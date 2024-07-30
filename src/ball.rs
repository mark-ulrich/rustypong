use std::rc::Rc;

use rand::Rng;

use crate::collidable::BoxCollidable;
use crate::color::Color;
use crate::gamemanager::GameManager;
use crate::math::Vec2f;
use crate::paddle::Paddle;
use crate::rect::Rect;

/// The default speed of the ball in pixels per second.
const DEFAULT_BALL_SPEED: f32 = 200.0;

/// The radius of the ball in pixels.
const BALL_RADIUS: f32 = 5.0;

/// The factor by which the ball speed increases when it hits a paddle.
const SPEEDUP_FACTOR: f32 = 1.05;

/// Defines the possible states of a ball.
#[derive(Debug)]
enum BallState {
    Idle,
    Moving,
}

/// A struct representing a ball in the game.
#[derive(Debug)]
pub struct Ball {
    game_manager: Rc<GameManager>,

    /// The current position of the ball.
    position: Vec2f,

    /// The bounds of the field.
    bounds: Rect,

    /// The radius of the ball.
    radius: f32,

    /// The current speed of the ball.
    speed: f32,

    /// The current heading of the ball.
    heading: f32,

    /// The current state of the ball.
    state: BallState,

    tags: Vec<String>,
}


impl Ball {
    /// Create a new ball at the given position.
    pub fn new(game_manager: GameManager, position: Vec2f) -> Self {
        let bounds = game_manager.get_field_bounds();
        Ball {
            game_manager: Rc::new(game_manager),
            tags: vec![String::from("ball")],
            bounds,
            position,
            heading: Self::generate_random_initial_ball_heading(),
            radius: BALL_RADIUS,
            speed: DEFAULT_BALL_SPEED,
            state: BallState::Idle,
        }
    }

    /// Update the ball. This should be called once per frame.
    ///
    /// # Arguments
    /// * `dt` - The time since the last frame in seconds.
    pub fn tick(&mut self, dt: f32) {
        match self.state {
            BallState::Moving => {
                let vec = Vec2f::from_angle(self.heading).normalized();
                let velocity = vec * self.speed * dt;
                self.position += velocity;

                // Bounce off the top and bottom of the field
                if self.position.y - self.radius < self.bounds.top {
                    self.heading = Vec2f::from_angle(self.heading).reflected(&Vec2f::DOWN).to_angle();
                } else if self.position.y + self.radius > self.bounds.bottom {
                    self.heading = Vec2f::from_angle(self.heading).reflect(&Vec2f::UP).to_angle();
                }

                // Bounce off the left and right of the screen
                if self.position.x - self.radius < self.bounds.left {
                    // self.heading = Vec2f::from_angle(self.heading).reflected(&Vec2f::RIGHT).to_angle();
                } else if self.position.x + self.radius > self.bounds.right {
                    // self.heading = Vec2f::from_angle(self.heading).reflect(&Vec2f::LEFT).to_angle();
                }
            }
            BallState::Idle => {}
        }
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let color = Color::from_hexstring("#fff");
        canvas.set_draw_color(color.to_sdl_color());

        let rect = sdl2::rect::FRect::new(
            self.position.x - self.radius,
            self.position.y - self.radius,
            self.radius * 2.0,
            self.radius * 2.0,
        );
        canvas.fill_frect(rect).unwrap();
    }

    pub fn start_moving(&mut self) {
        self.state = BallState::Moving;
    }

    pub fn stop_moving(&mut self) {
        self.state = BallState::Idle;
    }

    pub fn check_paddle_collisions(&mut self, paddles: &[&Paddle; 2]) {
        for &paddle in paddles {
            if self.get_bounds().collides_with(paddle.get_bounds()) {
                let paddle_collidable = Box::new(paddle.clone()) as Box<dyn BoxCollidable>;
                self.on_collision(&*paddle_collidable);
            }
        }
    }

    /// Generate a random heading for the ball to start moving in.
    /// The angle is guaranteed to be mostly horizontal.
    fn generate_random_initial_ball_heading() -> f32 {
        let angle = loop {
            let mut rng = rand::thread_rng();
            let angle = rng.gen_range(0.0..360.0);

            // ensure the ball is heading mostly towards the paddles
            let vec = Vec2f::from_angle(angle);
            let dot = vec.dot(&Vec2f::RIGHT);
            if dot.abs() > 0.5 {
                break angle;
            }
        };

        angle
    }
}

impl BoxCollidable for Ball {
    fn get_bounds(&self) -> Rect {
        Rect {
            left: self.position.x - self.radius,
            right: self.position.x + self.radius,
            top: self.position.y - self.radius,
            bottom: self.position.y + self.radius,
        }
    }

    fn on_collision(&mut self, other: &dyn BoxCollidable) {
        let other_tags = other.get_tags();
        if !other_tags.contains(&"paddle".to_string()) {
            return;
        }

        if other_tags.contains(&"paddle_left".to_string()) {
            dbg!("Ball collided with left paddle");
        } else if other_tags.contains(&"paddle_right".to_string()) {
            dbg!("Ball collided with right paddle");
        }

        // Determine the side of the paddle that was hit
        // WARNING: This is stupid hacky
        let paddle_top = Rect::new(
            other.get_bounds().left,
            other.get_bounds().top,
            other.get_bounds().right,
            other.get_bounds().top + 5.0,
        );
        let paddle_bottom = Rect::new(
            other.get_bounds().left,
            other.get_bounds().bottom - 5.0,
            other.get_bounds().right,
            other.get_bounds().bottom,
        );
        let paddle_left = Rect::new(
            other.get_bounds().left,
            other.get_bounds().top,
            other.get_bounds().left + 5.0,
            other.get_bounds().bottom,
        );
        let paddle_right = Rect::new(
            other.get_bounds().right - 5.0,
            other.get_bounds().top,
            other.get_bounds().right,
            other.get_bounds().bottom,
        );
        if self.get_bounds().collides_with(paddle_top) {
            self.heading = Vec2f::from_angle(self.heading).reflected(&Vec2f::UP).to_angle();
            self.position.y = paddle_top.top - self.radius;
        } else if self.get_bounds().collides_with(paddle_bottom) {
            self.heading = Vec2f::from_angle(self.heading).reflected(&Vec2f::DOWN).to_angle();
            self.position.y = paddle_bottom.bottom + self.radius;
        } else if self.get_bounds().collides_with(paddle_left) {
            self.heading = Vec2f::from_angle(self.heading).reflected(&Vec2f::LEFT).to_angle();
            self.position.x = paddle_left.left - self.radius;
        } else if self.get_bounds().collides_with(paddle_right) {
            self.heading = Vec2f::from_angle(self.heading).reflected(&Vec2f::RIGHT).to_angle();
            self.position.x = paddle_right.right + self.radius;
        }

        self.speed *= SPEEDUP_FACTOR;
    }

    fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }
}

impl Clone for BallState {
    fn clone(&self) -> BallState {
        match self {
            BallState::Idle => BallState::Idle,
            BallState::Moving => BallState::Moving,
        }
    }
}

impl Clone for Ball {
    fn clone(&self) -> Ball {
        Ball {
            game_manager: self.game_manager.clone(),
            tags: self.tags.clone(),
            position: self.position.clone(),
            bounds: self.bounds.clone(),
            radius: self.radius,
            speed: self.speed,
            heading: self.heading,
            state: self.state.clone(),
        }
    }
}
