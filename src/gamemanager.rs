use std::collections::HashMap;
use std::hash::Hash;

use crate::ball::Ball;
use crate::color::Color;
use crate::paddle::{Paddle, PaddleDirection, PaddleSide};
use crate::rect::Rect;
use crate::vec2f::Vec2f;

#[derive(Debug)]
pub struct GameManager {
    score: HashMap<String, i32>,
    paddles: HashMap<String, Paddle>,
    ball: Ball,
    field_bounds: Rect,
}

impl GameManager {
    pub fn new(field_bounds: Rect) -> Self {
        let mut game_manager = GameManager {
            score: HashMap::new(),
            field_bounds,
            paddles: Self::init_paddles(field_bounds),
            ball: Self::init_ball(GameManager::new(field_bounds.clone())),
        };
        game_manager.ball = Ball::new(game_manager.clone(), Vec2f::new(0.0, 0.0));
        game_manager
    }

    /// Resets the paddles and the ball, but keeps the scores
    pub fn reset_field(&mut self) {
        self.paddles = Self::init_paddles(self.field_bounds);
        let field_center = Vec2f::new(
            self.field_bounds.left + self.field_bounds.width() / 2.0,
            self.field_bounds.top + self.field_bounds.height() / 2.0,
        );

        self.ball = Ball::new(self.clone(), field_center);
    }

    /// Resets the game manager, clearing all scores
    pub fn full_reset(&mut self) {
        self.score.clear();
        self.reset_field();
    }

    pub fn tick(&mut self, dt: f32) {
        self.paddles.get_mut("left").unwrap().tick(dt);
        self.paddles.get_mut("right").unwrap().tick(dt);
        self.ball.tick(dt);
        self.ball.check_paddle_collisions(&[
            self.paddles.get("left").unwrap(),
            self.paddles.get("right").unwrap(),
        ]);
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        Self::render_field(canvas, self.field_bounds);
        for paddle in self.paddles.values() {
            paddle.render(canvas);
        }
        self.ball.render(canvas);
    }

    /// Renders the field to the screen
    fn render_field(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, bounds: Rect) {
        let field_color = Color::from_hexstring("#000");
        let (r, g, b) = field_color.get_rgb_u8();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
        canvas.fill_frect(bounds.get_sdl_frect()).unwrap();
    }

    /// Resets the game manager, clearing all scores
    pub fn get_score(&self, tag: &str) -> i32 {
        if !["left", "right"].contains(&tag) {
            panic!("Invalid tag: {}", tag);
        }
        *self.score.get(tag).unwrap()
    }

    pub fn add_score(&mut self, tag: &str, score: i32) {
        if !["left", "right"].contains(&tag) {
            panic!("Invalid tag: {}", tag);
        }
        let current_score = self.score.entry(tag.to_string()).or_insert(0);
        *current_score += score;
    }

    pub fn get_field_bounds(&self) -> Rect {
        self.field_bounds.clone()
    }

    pub fn get_paddle(&self, tag: &str) -> &Paddle {
        self.paddles.get(tag).unwrap()
    }

    pub fn get_ball(&self) -> &Ball {
        &self.ball
    }

    pub fn unpause(&mut self) {
        self.ball.start_moving();
    }

    pub fn move_paddle(&mut self, tag: &str, direction: PaddleDirection) {
        let paddle = self.paddles.get_mut(tag).unwrap();
        match direction {
            PaddleDirection::Up => paddle.move_up(),
            PaddleDirection::Down => paddle.move_down(),
        }
    }

    pub fn set_paddle_idle(&mut self, tag: &str) {
        self.paddles.get_mut(tag).unwrap().stop_moving();
    }

    fn init_paddles(bounds: Rect) -> HashMap<String, Paddle> {
        let left_paddle = Paddle::new(bounds, PaddleSide::Left);
        let right_paddle = Paddle::new(bounds, PaddleSide::Right);
        let mut paddles = HashMap::new();
        paddles.insert("left".to_string(), left_paddle);
        paddles.insert("right".to_string(), right_paddle);
        paddles
    }

    fn init_ball(game_manager: GameManager) -> Ball {
        let bounds = game_manager.get_field_bounds();
        let field_center = Vec2f::new(
            bounds.left + bounds.width() / 2.0,
            bounds.top + bounds.height() / 2.0,
        );
        Ball::new(game_manager, field_center)
    }
}

impl Clone for GameManager {
    fn clone(&self) -> GameManager {
        GameManager {
            score: self.score.clone(),
            paddles: self.paddles.clone(),
            ball: self.ball.clone(),
            field_bounds: self.field_bounds.clone(),
        }
    }
}