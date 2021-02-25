mod ray;
mod sphere;
mod texture;

use crate::{common::Point, geometry::Vector};
use ray::Ray;

pub trait TextureMaterial {
    fn diffusion(&self, p: Point) -> (f64, f64, f64);
    fn specularity(&self, p: Point) -> f64;
}

pub trait Object {
    fn intersects(&self, ray: Ray) -> bool;
    fn normal(&self, p: Point) -> Vector;
    fn diffusion(&self, p: Point) -> (f64, f64, f64);
    fn specularity(&self, p: Point) -> f64;
}
