use crate::common::Point;

use super::Light;

pub struct PointLight {
    pos: Point,
    intensity: (f64, f64, f64),
}

impl PointLight {
    pub fn new(pos: Point, intensity: (f64, f64, f64)) -> PointLight {
        PointLight { pos, intensity }
    }
}

impl Light for PointLight {
    fn pos(&self) -> Point {
        self.pos
    }

    fn intensity(&self) -> (f64, f64, f64) {
        self.intensity
    }
}
