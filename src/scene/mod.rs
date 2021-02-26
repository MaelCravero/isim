mod camera;
mod ray;
mod scene;
mod sphere;
mod texture;

use crate::{common::Point, geometry::Vector};

pub use camera::Camera;
pub use ray::Ray;
pub use scene::Scene;

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
