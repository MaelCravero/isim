mod camera;
pub mod light;
mod ray;
mod scene;
mod sphere;
pub mod texture;

use crate::{common::Point, geometry::Vector};

pub use camera::Camera;
pub use ray::Ray;
pub use scene::Scene;
pub use sphere::Sphere;

pub trait TextureMaterial {
    fn diffusion(&self, p: Point) -> (f64, f64, f64);
    fn specularity(&self, p: Point) -> f64;
}

pub trait Object {
    fn intersects(&self, ray: Ray) -> Option<f64>;
    fn normal(&self, p: Point) -> Vector;
    fn diffusion(&self, p: Point) -> (f64, f64, f64);
    fn specularity(&self, p: Point) -> f64;
}

pub trait Light {
    fn pos(&self) -> Point;
    fn intensity(&self) -> (f64, f64, f64);
}
