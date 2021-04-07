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

    fn is_inside(&self, p: Point) -> bool {
        let (a, b, c) = self.points;
        let pa = Vector::from(p, a);
        let pb = Vector::from(p, b);
        let pc = Vector::from(p, c);

        let ab = Vector::from(a, b);
        let bc = Vector::from(b, c);
        let ca = Vector::from(c, a);

        let n = self.normal(p).vector();
        let a = Vector::dot_product(&ab, &pa);
        let b = Vector::dot_product(&bc, &pb);
        let c = Vector::dot_product(&ca, &pc);

        (a >= 0.0 && b >= 0.0 && c >= 0.0) || (a <= 0.0 && b <= 0.0 && c <= 0.0)
    }
}

impl<T> Object for Triangle<T>
where
    T: TextureMaterial,
{
    fn intersects(&self, ray: Ray) -> Option<f64> {
        let (a, b, c) = self.points;
        let ab = Vector::from(a, b);
        let ac = Vector::from(a, c);

        let pvec = Vector::cross_product(&ray.direction.vector(), &ac);
        let det = Vector::dot_product(&ab, &pvec);

        if det < std::f64::EPSILON && det > -std::f64::EPSILON {
            return None;
        }

        let n = self.normal(ray.origin);
        if NormalVector::dot_product(&n, &ray.direction) > -std::f64::EPSILON
            && NormalVector::dot_product(&n, &ray.direction) < std::f64::EPSILON
        {
            return None;
        }

        let inv_det = 1.0 / det;
        let tvec = Vector::from(a, ray.origin);
        let u = inv_det * Vector::dot_product(&tvec, &pvec);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = Vector::cross_product(&tvec, &ab);
        let v = inv_det * Vector::dot_product(&ray.direction.vector(), &qvec);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = inv_det * Vector::dot_product(&ac, &qvec);
        if t < 0.0 {
            return None;
        }
        return Some(t);

        /*let plane_distance =
            Vector::dot_product(&n.vector(), &Vector::from(ORIGIN, self.points.0));
        let distance = Vector::dot_product(&n.vector(), &Vector::from(ORIGIN, ray.origin))
            + plane_distance / a;

        if distance < 0.0 {
            return None;
        }

        let intersection = Vector::from(ORIGIN, ray.origin) + ray.direction.vector() * distance;

        if self.is_inside(intersection.to_point()) {
            Some(distance)
        } else {
            None
        }*/
    }

    fn normal(&self, _p: Point) -> NormalVector {
        // Makes no use of p, might be a problem
        let (a, b, c) = self.points;
        let ab = Vector::from(a, b);
        let ac = Vector::from(a, c);
        Vector::cross_product(&ab, &ac).normalize()
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
