use crate::rect::Rect;
use crate::vec2f::Vec2f;

const DEFAULT_PADDLE_SPEED: f32 = 5.0;
const PADDLE_X_PADDING: f32 = 10.0;
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 60.0;

#[derive(Debug)]
pub enum PaddleSide {
    Left,
    Right,
}

#[derive(Debug, Copy)]
enum PaddleDirection {
    Up,
    Down,
}

impl Clone for PaddleDirection {
    fn clone(&self) -> PaddleDirection {
        match self {
            PaddleDirection::Up => PaddleDirection::Up,
            PaddleDirection::Down => PaddleDirection::Down,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum PaddleState {
    Idle,
    Moving(PaddleDirection),
}

#[derive(Debug)]
pub struct Paddle {
    position: Vec2f,
    speed: f32,
    side: PaddleSide,
    state: PaddleState,
}

impl Paddle {
    pub fn new(bounds: &Rect, side: PaddleSide) -> Self {
        let x_position = match side {
            PaddleSide::Left => bounds.left + PADDLE_X_PADDING + (PADDLE_WIDTH / 2.0),
            PaddleSide::Right => bounds.right - PADDLE_X_PADDING - (PADDLE_WIDTH / 2.0),
        };

        Paddle {
            position: Vec2f::new(x_position, bounds.height() / 2.0),
            speed: DEFAULT_PADDLE_SPEED,
            side,
            state: PaddleState::Idle,
        }
    }
}
