mod ray;
mod texture;

use crate::common::Point;

trait TextureMaterial {
    fn diffusion(&self, p: Point) -> (f64, f64, f64);
    fn specularity(&self, p: Point) -> f64;
}
