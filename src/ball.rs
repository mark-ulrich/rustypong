use rand::Rng;

use crate::collidable::BoxCollidable;
use crate::color::Color;
use crate::rect::Rect;
use crate::vec2f::Vec2f;

const DEFAULT_BALL_SPEED: f32 = 200.0;
const BALL_RADIUS: f32 = 5.0;

/// Defines the possible states of a ball.
#[derive(Debug)]
enum BallState {
    Idle,
    Moving,
}

/// A struct representing a ball in the game.
#[derive(Debug)]
pub struct Ball {
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
    /// Note: The bounds object is not used at this point.
    pub fn new(bounds: &Rect, position: Vec2f) -> Self {
        let _ = bounds; // Not used yet
        let heading = Self::generate_random_initial_ball_heading();

        Ball {
            tags: vec!["ball".to_string()],
            bounds: bounds.clone(),
            position,
            heading,
            radius: BALL_RADIUS,
            speed: DEFAULT_BALL_SPEED,
            state: BallState::Idle,
        }
    }

    /// Update the ball. This should be called once per frame.
    ///
    /// # Arguments
    /// * `dt` - The time since the last frame in seconds.
    pub fn update(&mut self, dt: f32) {
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
                    self.heading = Vec2f::from_angle(self.heading).reflected(&Vec2f::RIGHT).to_angle();
                } else if self.position.x + self.radius > self.bounds.right {
                    self.heading = Vec2f::from_angle(self.heading).reflect(&Vec2f::LEFT).to_angle();
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

    fn collides_with(&self, other: &dyn BoxCollidable) -> bool {
        let other_bounds = other.get_bounds();
        let ball_bounds = self.get_bounds();

        if ball_bounds.right < other_bounds.left {
            return false;
        }
        if ball_bounds.left > other_bounds.right {
            return false;
        }
        if ball_bounds.bottom < other_bounds.top {
            return false;
        }
        if ball_bounds.top > other_bounds.bottom {
            return false;
        }

        true
    }

    fn on_collision(&mut self, other: &dyn BoxCollidable) {
        let other_tags = other.get_tags();
        if !other_tags.contains(&"paddle".to_string()) {
            return;
        }

        // Determine the side of the paddle that was hit
        let paddle_bounds = other.get_bounds();
        let ball_bounds = self.get_bounds();
        let ball_center = Vec2f::new(ball_bounds.left + self.radius, ball_bounds.top + self.radius);
        let paddle_center = Vec2f::new(paddle_bounds.left + paddle_bounds.width() / 2.0, paddle_bounds.top + paddle_bounds.height() / 2.0);
        let diff = ball_center - paddle_center;

        // Reflect the ball off the paddle
        if diff.x.abs() > diff.y.abs() {
            self.heading = Vec2f::from_angle(self.heading).reflected(&Vec2f::RIGHT).to_angle();
        } else {
            self.heading = Vec2f::from_angle(self.heading).reflected(&Vec2f::DOWN).to_angle();
        }
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
