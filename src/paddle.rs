use sdl2::video::Window;

use crate::collidable::BoxCollidable;
use crate::color::Color;
use crate::rect::Rect;
use crate::vec2f::Vec2f;

/// Paddle speed in pixels per second
const DEFAULT_PADDLE_SPEED: f32 = 160.0;

/// Paddle width in pixels
const PADDLE_WIDTH: f32 = 10.0;

/// Paddle height in pixels
const PADDLE_HEIGHT: f32 = 60.0;

/// Padding from the screen edge to the paddle
const PADDLE_X_PADDING: f32 = 20.0;

/// Defines the side of the screen the paddle is on
#[derive(Debug)]
pub enum PaddleSide {
    Left,
    Right,
}

/// Defines the direction the paddle is moving
#[derive(Debug, Copy)]
enum PaddleDirection {
    Up,
    Down,
}

/// Defines the state of the paddle
#[derive(Debug, Copy, Clone)]
enum PaddleState {
    Idle,
    Moving(PaddleDirection),
}

/// Represents a paddle in the game
#[derive(Debug)]
pub struct Paddle {
    position: Vec2f,
    speed: f32,
    side: PaddleSide,
    state: PaddleState,
    bounds: Rect,
    tags: Vec<String>,
}

impl Paddle {
    pub fn new(bounds: &Rect, side: PaddleSide) -> Self {
        let x_position = match side {
            PaddleSide::Left => bounds.left + PADDLE_X_PADDING + (PADDLE_WIDTH / 2.0),
            PaddleSide::Right => bounds.right - PADDLE_X_PADDING - (PADDLE_WIDTH / 2.0),
        };
        let mut tags = vec!["paddle".to_string()];
        match side {
            PaddleSide::Left => tags.push("left_paddle".to_string()),
            PaddleSide::Right => tags.push("right_paddle".to_string()),
        }


        Paddle {
            tags,
            side,
            bounds: bounds.clone(),
            speed: DEFAULT_PADDLE_SPEED,
            state: PaddleState::Idle,
            position: Vec2f::new(x_position, bounds.top + bounds.height() / 2.0),
        }
    }

    /// Updates the paddle. This should be called once per frame.
    ///
    /// # Arguments
    /// * `dt` - Time since the last frame in seconds
    pub fn update(&mut self, dt: f32) {
        match self.state {
            PaddleState::Moving(direction) => {
                let heading_y = match direction {
                    PaddleDirection::Up => -1.0,
                    PaddleDirection::Down => 1.0,
                };
                let heading = Vec2f::new(0.0, heading_y);
                let velocity = heading * self.speed;
                self.position += velocity * dt;

                self.constrain_to_bounds();
            }
            PaddleState::Idle => {}
        }
    }

    /// Renders the paddle to the screen
    pub fn render(&self, canvas: &mut sdl2::render::Canvas<Window>) {
        let color = Color::from_hexstring("#fff");

        canvas.set_draw_color(color.to_sdl_color());
        let rect = sdl2::rect::FRect::new(
            self.position.x - (PADDLE_WIDTH / 2.0),
            self.position.y - (PADDLE_HEIGHT / 2.0),
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        );
        canvas.fill_frect(rect).unwrap();
    }

    /// Sets the paddle state to moving up
    pub fn move_up(&mut self) {
        self.state = PaddleState::Moving(PaddleDirection::Up);
    }

    /// Sets the paddle state to moving down
    pub fn move_down(&mut self) {
        self.state = PaddleState::Moving(PaddleDirection::Down);
    }

    /// Sets the paddle state to idle
    pub fn stop_moving(&mut self) {
        self.state = PaddleState::Idle;
    }

    /// Constrains the paddle to the bounds of the field
    fn constrain_to_bounds(&mut self) {
        let half_height = PADDLE_HEIGHT / 2.0;
        if self.position.y - half_height < self.bounds.top {
            self.position.y = self.bounds.top + half_height;
        } else if self.position.y + half_height > self.bounds.bottom {
            self.position.y = self.bounds.bottom - half_height;
        }
    }
}

impl BoxCollidable for Paddle {
    fn get_bounds(&self) -> Rect {
        Rect::new(
            self.position.x - (PADDLE_WIDTH / 2.0),
            self.position.y - (PADDLE_HEIGHT / 2.0),
            self.position.x + (PADDLE_WIDTH / 2.0),
            self.position.y + (PADDLE_HEIGHT / 2.0),
        )
    }

    fn collides_with(&self, other: &dyn BoxCollidable) -> bool {
        todo!()
    }

    fn on_collision(&mut self, other: &dyn BoxCollidable) {
        todo!()
    }

    fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }
}

impl Clone for PaddleSide {
    fn clone(&self) -> PaddleSide {
        match self {
            PaddleSide::Left => PaddleSide::Left,
            PaddleSide::Right => PaddleSide::Right,
        }
    }
}

impl Clone for PaddleDirection {
    fn clone(&self) -> PaddleDirection {
        match self {
            PaddleDirection::Up => PaddleDirection::Up,
            PaddleDirection::Down => PaddleDirection::Down,
        }
    }
}

impl Clone for Paddle {
    fn clone(&self) -> Paddle {
        Paddle {
            tags: self.tags.clone(),
            position: self.position.clone(),
            speed: self.speed,
            side: self.side.clone(),
            state: self.state.clone(),
            bounds: self.bounds.clone(),
        }
    }
}
