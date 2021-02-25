use super::TextureMaterial;
use crate::common::{Color, Point};

pub struct UniformTexture {
    color: Color,
    diff: f64,
    refl: f64,
}

impl UniformTexture {
    pub fn new(color: Color, diff: f64, refl: f64) -> UniformTexture {
        UniformTexture { color, diff, refl }
    }
}

impl TextureMaterial for UniformTexture {
    fn diffusion(&self, _p: Point) -> (f64, f64, f64) {
        let Color(r, g, b) = self.color;
        let (r, g, b) = (r as f64, g as f64, b as f64);
        (self.diff * r, self.diff * g, self.diff * b)
    }

    fn specularity(&self, _p: Point) -> f64 {
        self.refl
    }
}
