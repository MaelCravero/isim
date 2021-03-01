use crate::{common::Point, geometry::Vector};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub energy: f64,
    pub origin: Point,
    pub direction: Vector,
}
