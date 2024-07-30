pub struct GameManager {
    left_score: i32,
    right_score: i32,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            left_score: 0,
            right_score: 0,
        }
    }

    pub fn left_score(&self) -> i32 {
        self.left_score
    }

    pub fn right_score(&self) -> i32 {
        self.right_score
    }

    pub fn left_scored(&mut self) {
        self.left_score += 1;
    }

    pub fn right_scored(&mut self) {
        self.right_score += 1;
    }

    pub fn reset(&mut self) {
        self.left_score = 0;
        self.right_score = 0;
    }
}