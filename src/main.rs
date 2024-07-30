use std::collections::HashMap;

use color::Color;
use rect::Rect;

use crate::paddle::{Paddle, PaddleSide};

pub mod math;
pub mod ball;
pub mod color;
pub mod paddle;
pub mod rect;
pub mod boundingbox;
mod gamemanager;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

fn main() {
    let (mut canvas, mut event_pump) = init_sdl().unwrap_or_else(|e| {
        panic!("Error initializing SDL: {e}");
    });

    let field_y_padding = 40.0;
    let field_bounds = Rect {
        left: 0.0,
        right: SCREEN_WIDTH,
        top: field_y_padding,
        bottom: SCREEN_HEIGHT - field_y_padding,
    };

    let score: HashMap<String, i32> = HashMap::new();
    let (mut left_paddle, mut right_paddle) = (
        Paddle::new(field_bounds.clone(), PaddleSide::Left),
        Paddle::new(field_bounds.clone(), PaddleSide::Right),
    );
    let mut ball = ball::Ball::new(field_bounds.clone());

    let mut last_frame_time = std::time::Instant::now();

    ball.start_moving();

    'running: loop {
        let now = std::time::Instant::now();
        let dt = (now - last_frame_time).as_secs_f32();

        // ------------------- Handle Events -------------------
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                sdl2::event::Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    match keycode {
                        sdl2::keyboard::Keycode::W => {
                            left_paddle.move_up();
                        }
                        sdl2::keyboard::Keycode::S => {
                            left_paddle.move_down();
                        }
                        sdl2::keyboard::Keycode::Up => {
                            right_paddle.move_up();
                        }
                        sdl2::keyboard::Keycode::Down => {
                            right_paddle.move_down();
                        }
                        _ => {}
                    }
                }
                sdl2::event::Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    match keycode {
                        sdl2::keyboard::Keycode::W | sdl2::keyboard::Keycode::S => {
                            left_paddle.stop_moving();
                        }
                        sdl2::keyboard::Keycode::Up | sdl2::keyboard::Keycode::Down => {
                            right_paddle.stop_moving();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        // ------------------- End Handle Events -------------------


        // ------------------- Update -------------------
        left_paddle.tick(dt);
        right_paddle.tick(dt);
        ball.tick(dt);

        ball.check_paddle_collisions(&left_paddle, &right_paddle);
        match ball.check_goal() {
            Some(PaddleSide::Left) => {
                dbg!("Right player scored");
            }
            Some(PaddleSide::Right) => {
                dbg!("Left player scored");
            }
            None => {}
        }
        // ------------------- End Update -------------------


        // ------------------- Rendering -------------------
        let bg_color = Color::from_hexstring("#222");
        let (r, g, b) = bg_color.get_rgb_u8();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
        canvas.clear();

        render_field(&mut canvas, &field_bounds);
        left_paddle.render(&mut canvas);
        right_paddle.render(&mut canvas);
        ball.render(&mut canvas);

        canvas.present();
        // ------------------- End Rendering -------------------

        last_frame_time = now;
    }
}

type InitSdlResult = Result<(sdl2::render::Canvas<sdl2::video::Window>, sdl2::EventPump), String>;
/// Initializes SDL and creates a window and canvas, as well as an event pump
/// # Returns
/// A tuple containing the canvas and event pump, or an error message
fn init_sdl() -> InitSdlResult {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;

    let window = match video_subsystem
        .window("Rusty Pong", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .build()
    {
        Ok(window) => window,
        Err(_) => return Err(String::from("Failed to create window"))
    };

    let canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(_) => return Err(String::from("Failed to create canvas")),
    };

    let event_pump = sdl_context.event_pump()?;

    Ok((canvas, event_pump))
}


fn render_field(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, field_bounds: &Rect) {
    let field_color = Color::from_hexstring("#000");
    let (r, g, b) = field_color.get_rgb_u8();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, b));

    canvas.fill_frect(field_bounds.get_sdl_frect()).unwrap();
}
