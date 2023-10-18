use super::constants::INFINITY;

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn default() -> Self {
        Self {
            min: INFINITY,
            max: -INFINITY,
        }
    }

    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
}
