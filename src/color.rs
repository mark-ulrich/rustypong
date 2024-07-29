pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }

    pub fn from_hexstring(hex: &str) -> Self {
        let mut new_hex = hex;
        new_hex = new_hex.trim_start_matches('#');
        if new_hex.len() == 3 {
            let new_hex = hex
                .chars()
                .map(|c| c.to_string() + &c.to_string())
                .collect::<String>();
            return Self::from_hexstring(&new_hex);
        }

        let r = u8::from_str_radix(&new_hex[0..2], 16).unwrap() as f32 / 255.0;
        let g = u8::from_str_radix(&new_hex[2..4], 16).unwrap() as f32 / 255.0;
        let b = u8::from_str_radix(&new_hex[4..6], 16).unwrap() as f32 / 255.0;
        return Self::from_rgb(r, g, b);
    }

    pub fn to_sdl_color(&self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::RGB(
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        )
    }

    pub fn get_rgb_u8(&self) -> (u8, u8, u8) {
        (
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        )
    }
}
