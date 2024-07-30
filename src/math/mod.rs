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

    pub fn normalized(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.x /= magnitude;
        self.y /= magnitude;
    }

    /// Reflects the vector across the normal vector.
    /// Modifies the original vector in place.
    pub fn reflect(&mut self, normal: &Self) -> &Self {
        let dot = self.dot(normal);
        self.x -= 2.0 * dot * normal.x;
        self.y -= 2.0 * dot * normal.y;
        self
    }

    /// Returns a new vector that is the reflection of the original vector across the normal vector.
    pub fn reflected(&self, normal: &Self) -> Self {
        let dot = self.dot(normal);
        Self {
            x: self.x - 2.0 * dot * normal.x,
            y: self.y - 2.0 * dot * normal.y,
        }
    }
}

impl std::ops::Add for Vector2f {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for Vector2f {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::Add<f32> for Vector2f {
    type Output = Self;

    fn add(self, scalar: f32) -> Self {
        Self {
            x: self.x + scalar,
            y: self.y + scalar,
        }
    }
}

impl std::ops::AddAssign<f32> for Vector2f {
    fn add_assign(&mut self, scalar: f32) {
        self.x += scalar;
        self.y += scalar;
    }
}

impl std::ops::Sub for Vector2f {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::SubAssign for Vector2f {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl std::ops::Sub<f32> for Vector2f {
    type Output = Self;

    fn sub(self, scalar: f32) -> Self {
        Self {
            x: self.x - scalar,
            y: self.y - scalar,
        }
    }
}

impl std::ops::SubAssign<f32> for Vector2f {
    fn sub_assign(&mut self, scalar: f32) {
        self.x -= scalar;
        self.y -= scalar;
    }
}

impl std::ops::Mul for Vector2f {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl std::ops::MulAssign for Vector2f {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl std::ops::Mul<f32> for Vector2f {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl std::ops::MulAssign<f32> for Vector2f {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl std::ops::Div for Vector2f {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl std::ops::DivAssign for Vector2f {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

impl Clone for Vector2f {
    fn clone(&self) -> Vector2f {
        Vector2f {
            x: self.x,
            y: self.y,
        }
    }
}
