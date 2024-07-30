use collidable::BoxCollidable;
use color::Color;
use rect::Rect;

pub mod ball;
pub mod color;
pub mod paddle;
pub mod rect;
pub mod vec2f;
pub mod collidable;
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
    let mut game_manager = gamemanager::GameManager::new(field_bounds.clone());

    let mut last_frame_time = std::time::Instant::now();
    let mut dt = 0.0;

    game_manager.reset_field();
    game_manager.unpause();

    'running: loop {
        let now = std::time::Instant::now();
        dt = (now - last_frame_time).as_secs_f32();

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
                            game_manager.move_paddle("left", paddle::PaddleDirection::Up);
                        }
                        sdl2::keyboard::Keycode::S => {
                            game_manager.move_paddle("left", paddle::PaddleDirection::Down);
                        }
                        sdl2::keyboard::Keycode::Up => {
                            game_manager.move_paddle("right", paddle::PaddleDirection::Up);
                        }
                        sdl2::keyboard::Keycode::Down => {
                            game_manager.move_paddle("right", paddle::PaddleDirection::Down);
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
                            game_manager.set_paddle_idle("left");
                        }
                        sdl2::keyboard::Keycode::Up | sdl2::keyboard::Keycode::Down => {
                            game_manager.set_paddle_idle("right");
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        // ------------------- End Handle Events -------------------


        // ------------------- Update -------------------
        game_manager.tick(dt);
        // ------------------- End Update -------------------


        // ------------------- Rendering -------------------
        let bg_color = Color::from_hexstring("#222");
        let (r, g, b) = bg_color.get_rgb_u8();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
        canvas.clear();
        game_manager.render(&mut canvas);
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
        Err(e) => return Err(String::from("Failed to create window"))
    };

    let canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(e) => return Err(String::from("Failed to create canvas")),
    };

    let event_pump = sdl_context.event_pump()?;

    Ok((canvas, event_pump))
}


