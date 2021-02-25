use crate::{
    common::{Color, Point},
    geometry::Vector,
};

pub struct Ray {
    color: Color,
    origin: Point,
    direction: Vector,
}
