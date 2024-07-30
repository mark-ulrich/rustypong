use rand::Rng;

use crate::boundingbox::BoundingBox;
use crate::color::Color;
use crate::math::Vec2f;
use crate::paddle::{Paddle, PaddleSide};
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
    /// The current position of the ball.
    position: Vec2f,

    /// The bounds of the field.
    field_bounds: Rect,

    /// The radius of the ball.
    radius: f32,

    /// The current speed of the ball.
    speed: f32,

    /// The current heading of the ball.
    heading: f32,

    /// The current state of the ball.
    state: BallState,
}


impl Ball {
    /// Create a new ball at the given position.
    pub fn new(bounds: Rect) -> Self {
        Ball {
            field_bounds: bounds,
            position: bounds.center(),
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
                if self.position.y - self.radius < self.field_bounds.top {
                    self.heading = Vec2f::from_angle(self.heading).reflected(&Vec2f::DOWN).to_angle();
                } else if self.position.y + self.radius > self.field_bounds.bottom {
                    self.heading = Vec2f::from_angle(self.heading).reflect(&Vec2f::UP).to_angle();
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

    pub fn check_paddle_collisions(&mut self, left_paddle: &Paddle, right_paddle: &Paddle) {
        let left_paddle_bounds = left_paddle.get_bounds();
        let right_paddle_bounds = right_paddle.get_bounds();

        let ball_bounds = Rect::new(
            self.position.x - self.radius,
            self.position.y - self.radius,
            self.position.x + self.radius,
            self.position.y + self.radius,
        );

        if ball_bounds.intersects(&left_paddle_bounds) {
            dbg!("left paddle hit");
            self.heading = Vec2f::from_angle(self.heading).reflected(&Vec2f::RIGHT).to_angle();
            self.speed *= SPEEDUP_FACTOR;
        } else if ball_bounds.intersects(&right_paddle_bounds) {
            dbg!("right paddle hit");
            self.heading = Vec2f::from_angle(self.heading).reflect(&Vec2f::LEFT).to_angle();
            self.speed *= SPEEDUP_FACTOR;
        }
    }

    pub fn check_goal(&mut self) -> Option<PaddleSide> {
        if self.position.x - self.radius < self.field_bounds.left {
            return Some(PaddleSide::Right);
        } else if self.position.x + self.radius > self.field_bounds.right {
            return Some(PaddleSide::Left);
        }
        None
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
            if dot.abs() > 0.4 {
                break angle;
            }
        };

        angle
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
            position: self.position.clone(),
            field_bounds: self.field_bounds.clone(),
            radius: self.radius,
            speed: self.speed,
            heading: self.heading,
            state: self.state.clone(),
        }
    }
}
