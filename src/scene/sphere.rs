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
    fn intersects(&self, ray: Ray) -> bool {
        let v = Vector::from(self.center, ray.origin);
        let a = Vector::dot_product(&ray.direction, &ray.direction);
        let b = 2.0 * Vector::dot_product(&v, &ray.direction);
        let c = Vector::dot_product(&v, &v) - self.radius * self.radius;

        return (b * b - 4.0 * a * c) / (2.0 * a) >= 0.0;
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

#[cfg(test)]
mod tests {
    use crate::common::{Color, ORIGIN, WHITE};
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
            color: WHITE,
            origin: ORIGIN,
            direction: Vector::new(1.0, 0.0, 0.0),
        };

        assert!(s.intersects(ray))
    }

    #[test]
    fn intersects_false() {
        let s = Sphere::new(
            Point(5.0, 5.0, 5.0),
            1.0,
            UniformTexture::new(WHITE, 0.0, 0.0),
        );

        let ray = Ray {
            color: WHITE,
            origin: ORIGIN,
            direction: Vector::new(1.0, 0.0, 0.0),
        };

        assert!(!s.intersects(ray))
    }
}
