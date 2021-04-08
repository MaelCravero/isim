use crate::common::*;

use super::{Object, Ray, TextureMaterial};

#[derive(Debug, Clone)]
pub struct Cylinder<T: TextureMaterial> {
    a: Point,
    b: Point,
    r: f64,
    texture: T,
}

impl<T> Cylinder<T>
where
    T: TextureMaterial,
{
    pub fn new(a: Point, b: Point, r: f64, texture: T) -> Cylinder<T> {
        Cylinder { a, b, r, texture }
    }
}

impl<T> Object for Cylinder<T>
where
    T: TextureMaterial,
{
    fn normal(&self, p: Point) -> NormalVector {
        unimplemented!()
    }

    fn intersects(&self, ray: Ray) -> Option<f64> {
        unimplemented!()
    }

    fn specularity(&self, p: Point) -> f64 {
        self.texture.specularity(p)
    }

    fn diffusion(&self, p: Point) -> (f64, f64, f64) {
        self.texture.diffusion(p)
    }
}
