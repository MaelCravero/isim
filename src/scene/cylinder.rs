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
        let n = Vector::from(self.a, self.b).normalize();
        let dot = Vector::dot_product(&Vector::from(self.a, p), &n.vector());
        let axis_p = (Vector::from(ORIGIN, self.a) + dot * n.vector()).to_point();

        Vector::from(axis_p, p).normalize()
    }

    fn intersects(&self, ray: Ray) -> Option<f64> {
        let direction = Vector::from(self.a, self.b).normalize();

        let d = Vector::from(self.a, ray.origin);

        let v = ray.direction.vector();
        let w = direction.vector();

        let dot_vw = Vector::dot_product(&v, &w);
        let dot_dw = Vector::dot_product(&d, &w);

        let sub_a = v - w * dot_vw;
        let a = Vector::dot_product(&sub_a, &sub_a);

        let b = 2.0 * Vector::dot_product(&(v - w * dot_vw), &(d - w * dot_dw));

        let sub_c = d - w * dot_dw;
        let c = Vector::dot_product(&sub_c, &sub_c) - (self.r * self.r);

        let mut intersections = Vec::new();

        let delta = (b * b - 4.0 * a * c) / (2.0 * a);

        if delta >= 0.0 {
            let x1 = (-b + delta.sqrt()) / (2.0 * a);
            let x2 = (-b - delta.sqrt()) / (2.0 * a);

            let p1 = (Vector::from(ORIGIN, ray.origin) + x1 * ray.direction.vector()).to_point();
            let p2 = (Vector::from(ORIGIN, ray.origin) + x2 * ray.direction.vector()).to_point();

            let mut mark = |x, p| {
                if x > 0.0
                    && Vector::dot_product(&w, &Vector::from(p, self.a))
                        * Vector::dot_product(&w, &Vector::from(p, self.b))
                        < 0.0
                {
                    intersections.push(x);
                }
            };

            mark(x1, p1);
            mark(x2, p2);
        }

        let dot = NormalVector::dot_product(&direction, &ray.direction);
        if dot > std::f64::EPSILON {
            let mut mark = |p| {
                let t = Vector::dot_product(&Vector::from(ray.origin, p), &w) / dot;

                if t >= 0.0 {
                    let intersection =
                        (Vector::from(ORIGIN, ray.origin) + t * ray.direction.vector()).to_point();
                    let v = Vector::from(p, intersection);
                    if Vector::dot_product(&v, &v).sqrt() <= self.r {
                        intersections.push(t);
                    }
                }
            };

            mark(self.a);
            mark(self.b);
        }

        intersections.iter().map(|&f| f).fold(None, |acc, x| {
            Some(acc.map_or(x, |v| if v < x { v } else { x }))
        })
    }

    fn specularity(&self, p: Point) -> f64 {
        self.texture.specularity(p)
    }

    fn diffusion(&self, p: Point) -> (f64, f64, f64) {
        self.texture.diffusion(p)
    }
}
