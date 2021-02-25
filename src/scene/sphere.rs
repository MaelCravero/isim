use crate::{common::Point, geometry::Vector};

use super::{Object, Ray, TextureMaterial};

pub struct Sphere<T: TextureMaterial> {
    center: Point,
    radius: f64,
    texture: T,
}

impl<T> Sphere<T>
where
    T: TextureMaterial,
{
    fn new(center: Point, radius: f64, texture: T) -> Sphere<T> {
        Sphere {
            center,
            radius,
            texture,
        }
    }
}

impl<T> Object for Sphere<T>
where
    T: TextureMaterial,
{
    fn intersects(&self, ray: Ray) -> bool {
        unimplemented!()
    }

    fn normal(&self, p: Point) -> Vector {
        unimplemented!()
    }

    fn diffusion(&self, p: Point) -> (f64, f64, f64) {
        self.texture.diffusion(p)
    }

    fn specularity(&self, p: Point) -> f64 {
        self.texture.specularity(p)
    }
}
