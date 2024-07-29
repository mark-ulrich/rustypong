use ball::Ball;
use color::Color;
use paddle::{Paddle, PaddleSide};
use rect::Rect;
use vec2f::Vec2f;

pub mod ball;
pub mod color;
pub mod paddle;
pub mod rect;
pub mod vec2f;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

fn main() {
    let (mut canvas, mut event_pump) = init_sdl();

    let field_y_padding = 40.0;
    let bounds = Rect {
        left: 0.0,
        right: SCREEN_WIDTH,
        top: field_y_padding,
        bottom: SCREEN_HEIGHT - field_y_padding,
    };

    let (left_paddle, right_paddle) = init_paddles(&bounds);
    let ball_position = Vec2f::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
    let ball = Ball::new(&bounds, ball_position);
    println!("Left Paddle: {:?}", left_paddle);
    println!("Right Paddle: {:?}", right_paddle);
    println!("Ball: {:?}", ball);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                sdl2::event::Event::KeyUp {
                    timestamp: _timestamp,
                    window_id: _window_id,
                    keycode,
                    scancode: _scancode,
                    keymod: _keymod,
                    repeat: _repeat,
                } => match keycode {
                    Some(sdl2::keyboard::Keycode::Escape) => break 'running,
                    _ => {}
                },
                _ => {}
            }
        }

        let bg_color = Color::from_hexstring("#222");
        let (r, g, b) = bg_color.get_rgb_u8();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
        canvas.clear();

        draw_field(&mut canvas, &bounds);

        canvas.set_draw_color(Color::from_hexstring("#f00").to_sdl_color());
        canvas
            .fill_frect(Rect::new(100.0, 100.0, 200.0, 200.0).get_sdl_frect())
            .unwrap();

        canvas.present();
    }
}

fn init_paddles(bounds: &Rect) -> (Paddle, Paddle) {
    let left_paddle = Paddle::new(bounds, PaddleSide::Left);
    let right_paddle = Paddle::new(bounds, PaddleSide::Right);

    (left_paddle, right_paddle)
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
        .window("Pong", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .build()
    {
        Ok(window) => window,
        Err(e) => panic!("Failed to create window: {}", e),
    };

    let mut canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(e) => panic!("Failed to build canvas: {}", e),
    };

    let event_pump = match sdl_context.event_pump() {
        Ok(event_pump) => event_pump,
        Err(e) => panic!("Failed to get event pump: {}", e),
    };

    (canvas, event_pump)
}

fn draw_field(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, bounds: &Rect) {
    let field_color = Color::from_hexstring("#000");
    let (r, g, b) = field_color.get_rgb_u8();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
    canvas.fill_frect(bounds.get_sdl_frect()).unwrap();
}
