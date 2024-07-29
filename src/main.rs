use ball::Ball;
use collidable::BoxCollidable;
use color::Color;
use paddle::{Paddle, PaddleSide};
use rect::Rect;
use vec2f::Vec2f;

pub mod ball;
pub mod color;
pub mod paddle;
pub mod rect;
pub mod vec2f;
pub mod collidable;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

fn main() {
    let (mut canvas, mut event_pump) = init_sdl();

    let field_y_padding = 40.0;
    let field_bounds = Rect {
        left: 0.0,
        right: SCREEN_WIDTH,
        top: field_y_padding,
        bottom: SCREEN_HEIGHT - field_y_padding,
    };

    let mut collidables: Vec<&dyn BoxCollidable> = Vec::new();

    let (mut left_paddle, mut right_paddle) = init_paddles(&field_bounds);
    let field_center = Vec2f::new(
        field_bounds.left + field_bounds.width() / 2.0,
        field_bounds.top + field_bounds.height() / 2.0,
    );
    let mut ball = Ball::new(&field_bounds, field_center);

    // Add paddles and ball BoxCollidable trait objects to the collidables vector
    collidables.push(&mut left_paddle);
    collidables.push(&mut right_paddle);
    collidables.push(&mut ball);


    let mut last_frame_time = std::time::Instant::now();
    let mut dt = 0.0;
    ball.start_moving();
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


        // ------------------- Update -------------------
        left_paddle.update(dt);
        right_paddle.update(dt);
        ball.update(dt);


        // ------------------- Rendering -------------------
        let bg_color = Color::from_hexstring("#222");
        let (r, g, b) = bg_color.get_rgb_u8();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
        canvas.clear();

        draw_field(&mut canvas, &field_bounds);
        left_paddle.render(&mut canvas);
        right_paddle.render(&mut canvas);
        ball.render(&mut canvas);

        canvas.present();

        last_frame_time = now;
    }

    collidables.clear();
}

fn init_sdl() -> (sdl2::render::Canvas<sdl2::video::Window>, sdl2::EventPump) {
    let sdl_context = match sdl2::init() {
        Ok(sdl_context) => sdl_context,
        Err(e) => panic!("Failed to initialize SDL: {}", e),
    };

    let video_subsystem = match sdl_context.video() {
        Ok(video_subsystem) => video_subsystem,
        Err(e) => panic!("Failed to initialize video subsystem: {}", e),
    };

    let window = match video_subsystem
        .window("Rusty Pong", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .build()
    {
        Ok(window) => window,
        Err(e) => panic!("Failed to create window: {}", e),
    };

    let canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(e) => panic!("Failed to build canvas: {}", e),
    };

    let event_pump = match sdl_context.event_pump() {
        Ok(event_pump) => event_pump,
        Err(e) => panic!("Failed to get event pump: {}", e),
    };

    (canvas, event_pump)
}

fn init_paddles(bounds: &Rect) -> (Paddle, Paddle) {
    let left_paddle = Paddle::new(bounds, PaddleSide::Left);
    let right_paddle = Paddle::new(bounds, PaddleSide::Right);

    (left_paddle, right_paddle)
}

fn draw_field(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, bounds: &Rect) {
    let field_color = Color::from_hexstring("#222");
    let (r, g, b) = field_color.get_rgb_u8();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
    canvas.fill_frect(bounds.get_sdl_frect()).unwrap();
}

fn check_collisions(collidables: &mut Vec<Box<&mut dyn BoxCollidable>>) {
    for i in 0..collidables.len() {
        for j in i + 1..collidables.len() {
            let mut a = collidables[i];
            let mut b = collidables[j];

            if a.collides_with(&**b) {
                a.on_collision(&**b);
                b.on_collision(&**a);
            }
        }
    }
}
