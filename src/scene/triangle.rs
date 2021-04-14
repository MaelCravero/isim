use {super::Ray, crate::common::*};

use super::{Object, TextureMaterial};

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

impl<T> Object for Triangle<T>
where
    T: TextureMaterial,
{
    fn intersects(&self, ray: Ray) -> Option<f64> {
        // Moeller-Trumbore algorithm

        let (a, b, c) = self.points;
        let ab = Vector::from(a, b);
        let ac = Vector::from(a, c);

        let h = Vector::cross_product(&ray.direction.vector(), &ac);
        let dot = Vector::dot_product(&ab, &h);

        if dot < std::f64::EPSILON && dot > -std::f64::EPSILON {
            return None;
        }

        let n = self.normal(ray.origin);
        if NormalVector::dot_product(&n, &ray.direction) > -std::f64::EPSILON
            && NormalVector::dot_product(&n, &ray.direction) < std::f64::EPSILON
        {
            return None;
        }

        let inv_dot = 1.0 / dot;
        let ao = Vector::from(a, ray.origin);
        let u = inv_dot * Vector::dot_product(&ao, &h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let cross = Vector::cross_product(&ao, &ab);
        let v = inv_dot * Vector::dot_product(&ray.direction.vector(), &cross);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = inv_dot * Vector::dot_product(&ac, &cross);
        if t < 0.0 {
            return None;
        }
        return Some(t);
    }

    fn normal(&self, _p: Point) -> NormalVector {
        let (a, b, c) = self.points;
        let ab = Vector::from(a, b);
        let ac = Vector::from(a, c);
        Vector::cross_product(&ab, &ac).normalize()
    }

    fn diffusion(&self, _p: Point) -> (f64, f64, f64) {
        self.texture.diffusion(0.0, 0.0)
    }

    fn specularity(&self, _p: Point) -> f64 {
        self.texture.specularity(0, 0)
    }

    fn map_to_texture(&self, p: Point) -> (f64, f64) {
        (0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{ORIGIN, WHITE};
    use crate::scene::texture::UniformTexture;

    use super::*;

    #[test]
    fn intersects_true() {
        let t = Triangle::new(
            (
                Point(1.0, 0.0, 5.0),
                Point(-1.0, -1.0, 5.0),
                Point(-1.0, 1.0, 5.0),
            ),
            UniformTexture::new(WHITE, 0.0, 0.0),
        );

        let ray = Ray {
            energy: 1.0,
            origin: ORIGIN,
            direction: Vector::new(0.0, 0.0, 1.0).normalize(),
        };

        assert!(t.intersects(ray).is_some())
    }

    #[test]
    fn intersects_false_colinear() {
        let t = Triangle::new(
            (
                Point(-0.5, 0.0, 5.0),
                Point(0.5, 1.0, 5.0),
                Point(0.5, -1.0, 5.0),
            ),
            UniformTexture::new(WHITE, 0.0, 0.0),
        );

        let ray = Ray {
            energy: 1.0,
            origin: ORIGIN,
            direction: Vector::new(1.0, 0.0, 0.0).normalize(),
        };

        assert!(t.intersects(ray).is_none())
    }

    #[test]
    fn intersects_false_non_colinear() {
        let t = Triangle::new(
            (
                Point(-0.5, 0.0, 5.0),
                Point(0.5, 1.0, 5.0),
                Point(0.5, -1.0, 5.0),
            ),
            UniformTexture::new(WHITE, 0.0, 0.0),
        );

        let ray = Ray {
            energy: 1.0,
            origin: Point(2.0, 2.0, 0.0),
            direction: Vector::new(0.0, 0.0, 5.0).normalize(),
        };

        assert!(t.intersects(ray).is_none())
    }
}
