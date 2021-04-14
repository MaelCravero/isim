use crate::common::*;
use std::cell::UnsafeCell;

use super::{Object, Ray, TextureMaterial};

pub struct Cylinder<T: TextureMaterial> {
    a: Point,
    b: Point,
    r: f64,
    texture: T,
    direction: NormalVector,
    ref_normal: UnsafeCell<Option<NormalVector>>,
}

impl<T> Cylinder<T>
where
    T: TextureMaterial,
{
    pub fn new(a: Point, b: Point, r: f64, texture: T) -> Cylinder<T> {
        let direction = Vector::from(a, b).normalize();
        Cylinder {
            a,
            b,
            r,
            texture,
            direction,
            ref_normal: UnsafeCell::new(None),
        }
    }
}

impl<T> Object for Cylinder<T>
where
    T: TextureMaterial,
{
    fn normal(&self, p: Point) -> NormalVector {
        let n = self.direction.vector();
        let dot = Vector::dot_product(&Vector::from(self.a, p), &n);
        let axis_p = (Vector::from(ORIGIN, self.a) + dot * n).to_point();

        Vector::from(axis_p, p).normalize()
    }

    fn intersects(&self, ray: Ray) -> Option<f64> {
        let d = Vector::from(self.a, ray.origin);

        let origin = Vector::from(ORIGIN, ray.origin);

        let v = ray.direction.vector();
        let w = self.direction.vector();

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

            let mut mark = |x| {
                let p = (origin + x * ray.direction.vector()).to_point();
                if Vector::dot_product(&w, &Vector::from(p, self.a))
                    * Vector::dot_product(&w, &Vector::from(p, self.b))
                    < 0.0
                {
                    intersections.push(x);
                }
            };

            if x1 > 0.0 {
                mark(x1);
            }

            if x2 > 0.0 {
                mark(x2);
            }

            let dot = NormalVector::dot_product(&self.direction, &ray.direction);
            if dot > std::f64::EPSILON {
                let mut mark = |p| {
                    let t = Vector::dot_product(&Vector::from(ray.origin, p), &w) / dot;

                    if t >= 0.0 {
                        let intersection = (origin + t * ray.direction.vector()).to_point();
                        let v = Vector::from(p, intersection);
                        if Vector::dot_product(&v, &v).sqrt() <= self.r {
                            intersections.push(t);
                        }
                    }
                };

                mark(self.a);
                mark(self.b);
            }
        }
        intersections.iter().map(|&f| f).fold(None, |acc, x| {
            Some(acc.map_or(x, |v| if v < x { v } else { x }))
        })
    }

    fn specularity(&self, p: Point) -> f64 {
        self.texture.specularity(0, 0)
    }

    fn diffusion(&self, p: Point) -> (f64, f64, f64) {
        let (u, v) = self.map_to_texture(p);
        self.texture.diffusion(u, v)
    }

    fn map_to_texture(&self, p: Point) -> (f64, f64) {
        unsafe {
            let n = self.direction.vector();
            let ptr = self.ref_normal.get();
            if (*ptr).is_none() {
                *ptr = Some(Vector::cross_product(&self.normal(p).vector(), &n).normalize())
            }
            let v = Vector::dot_product(&Vector::from(self.a, p), &n)
                / (Vector::from(self.a, self.b).norm());
            let u = NormalVector::dot_product(&(*ptr).unwrap(), &self.normal(p)).acos()
                / (std::f64::consts::PI);
            (u, v)
        }
    }
}
