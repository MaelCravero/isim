use crate::common::*;

use super::TextureMaterial;

type PointTriplet = (Point, Point, Point);

pub struct Triangle<T: TextureMaterial> {
    points: PointTriplet,
    texture: T,
}

impl<T> Triangle<T>
where
    T: TextureMaterial,
{
    pub fn new(points: PointTriplet, texture: T) -> Triangle<T> {
        Triangle { points, texture }
    }
}
