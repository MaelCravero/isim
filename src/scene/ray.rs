use crate::{
    common::{Color, Point},
    geometry::Vector,
};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub color: Color,
    pub origin: Point,
    pub direction: Vector,
}
