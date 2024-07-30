#[derive(Debug, Copy)]
pub struct Rect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Rect {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn width(&self) -> f32 {
        self.right - self.left
    }
    pub fn height(&self) -> f32 {
        self.bottom - self.top
    }

    pub fn get_sdl_rect(&self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(
            self.left as i32,
            self.top as i32,
            self.width() as u32,
            self.height() as u32,
        )
    }

    pub fn get_sdl_frect(&self) -> sdl2::rect::FRect {
        sdl2::rect::FRect::new(self.left, self.top, self.width(), self.height())
    }

    pub fn collides_with(&self, other: Rect) -> bool {
        self.left < other.right && self.right > other.left && self.top < other.bottom && self.bottom > other.top
    }
}

impl Clone for Rect {
    fn clone(&self) -> Rect {
        Rect {
            left: self.left,
            right: self.right,
            top: self.top,
            bottom: self.bottom,
        }
    }
}

