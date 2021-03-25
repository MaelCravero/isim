use crate::{common::Point, geometry::NormalVector};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub energy: f64,
    pub origin: Point,
    pub direction: NormalVector,
}

impl Ray {
    pub fn reflected(&self, normal: &NormalVector) -> NormalVector {
        let reflected = self.direction.vector()
            - normal.vector() * 2.0 * NormalVector::dot_product(&normal, &self.direction);

        debug_assert!(1.0 - reflected.norm() < 0.001);

        reflected.normalize()
    }
}
