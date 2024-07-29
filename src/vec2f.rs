#[derive(Debug)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}
pub type Vec2f = Vector2f;

impl Vector2f {
    pub const ZERO: Vector2f = Vector2f { x: 0.0, y: 0.0 };
    pub const ONE: Vector2f = Vector2f { x: 1.0, y: 1.0 };

    pub const UP: Vector2f = Vector2f { x: 0.0, y: -1.0 };
    pub const DOWN: Vector2f = Vector2f { x: 0.0, y: 1.0 };
    pub const LEFT: Vector2f = Vector2f { x: -1.0, y: 0.0 };
    pub const RIGHT: Vector2f = Vector2f { x: 1.0, y: 0.0 };

    pub const NORTH: Vector2f = Vector2f { x: 0.0, y: -1.0 };
    pub const EAST: Vector2f = Vector2f { x: 1.0, y: 0.0 };
    pub const SOUTH: Vector2f = Vector2f { x: 0.0, y: 1.0 };
    pub const WEST: Vector2f = Vector2f { x: -1.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Vector2f { x, y }
    }

    pub fn from_angle(angle: f32) -> Self {
        Vector2f {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    pub fn to_angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn distance_to(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}
