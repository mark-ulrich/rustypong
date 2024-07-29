use crate::rect::Rect;
use crate::vec2f::Vec2f;

use rand::Rng;

const DEFAULT_BALL_SPEED: f32 = 5.0;
const BALL_RADIUS: f32 = 10.0;

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

    /// The current speed of the ball.
    speed: f32,

    /// The current heading of the ball.
    heading: f32,

    /// The current state of the ball.
    state: BallState,
}

impl Ball {
    /// Create a new ball at the given position.
    /// Note: The bounds object is not used at this point.
    pub fn new(bounds: &Rect, position: Vec2f) -> Self {
        let _ = bounds; // Not used yet
        let heading = Self::generate_random_initial_ball_heading();

        Ball {
            position,
            heading,
            speed: DEFAULT_BALL_SPEED,
            state: BallState::Idle,
        }
    }

    /// Generate a random angle for the ball to start moving in.
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
