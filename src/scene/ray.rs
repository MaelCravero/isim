use crate::{
    common::{Color, Point},
    geometry::Vector,
};

pub struct Ray {
    pub color: Color,
    pub origin: Point,
    pub direction: Vector,
}
