use crate::{
    common::{Point, ORIGIN},
    geometry::NormalVector,
    geometry::Vector,
};

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
    pub fn new(center: Point, radius: f64, texture: T) -> Sphere<T> {
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
    fn intersects(&self, ray: Ray) -> Option<f64> {
        let v = Vector::from(self.center, ray.origin);
        let a = NormalVector::dot_product(&ray.direction, &ray.direction);
        let b = 2.0 * Vector::dot_product(&v, &ray.direction.vector());
        let c = Vector::dot_product(&v, &v) - self.radius * self.radius;

        let delta = (b * b - 4.0 * a * c) / (2.0 * a);

        if delta >= 0.0 {
            let x1 = (-b + delta.sqrt()) / (2.0 * a);
            let x2 = (-b - delta.sqrt()) / (2.0 * a);

            // We want the shortest distance
            let (min, max) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            if min < 0.0 && max < 0.0 {
                None
            } else if min > 0.0 {
                Some(min)
            } else {
                Some(max) // If the camera is inside an object
            }
        } else {
            None
        }
    }

    fn normal(&self, p: Point) -> NormalVector {
        Vector::from(self.center, p).normalize()
    }

    fn diffusion(&self, p: Point) -> (f64, f64, f64) {
        let (x, y) = self.map_to_texture(p);
        self.texture.diffusion(x, y)
    }

    fn specularity(&self, _p: Point) -> f64 {
        self.texture.specularity(0, 0)
    }

    fn map_to_texture(&self, p: Point) -> (f64, f64) {
        let n = Vector::from(self.center, p).normalize().vector();
        let u = 0.5 + n.z.atan2(n.y) * (1.0 / (2.0 * std::f64::consts::PI));
        let v = 0.5 - n.x.asin() * (1.0 / std::f64::consts::PI);
        (u, v)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{ORIGIN, WHITE};
    use crate::scene::texture::UniformTexture;

    use super::*;

    #[test]
    fn intersects_true() {
        let s = Sphere::new(
            Point(2.0, 0.0, 0.0),
            1.0,
            UniformTexture::new(WHITE, 0.0, 0.0),
        );

        let ray = Ray {
            energy: 1.0,
            origin: ORIGIN,
            direction: Vector::new(1.0, 0.0, 0.0).normalize(),
        };

        assert!(s.intersects(ray).is_some())
    }

    #[test]
    fn intersects_false() {
        let s = Sphere::new(
            Point(5.0, 5.0, 5.0),
            1.0,
            UniformTexture::new(WHITE, 0.0, 0.0),
        );

        let ray = Ray {
            energy: 1.0,
            origin: ORIGIN,
            direction: Vector::new(1.0, 0.0, 0.0).normalize(),
        };

        assert!(s.intersects(ray).is_none())
    }
}
