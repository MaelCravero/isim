use std::collections::HashMap;

use crate::{
    common::Color,
    geometry::Vector,
    image::Image,
    scene::{Ray, Scene},
};

pub struct Engine {
    scene: Scene,
}

impl Engine {
    pub fn new(scene: Scene) -> Engine {
        Engine { scene }
    }
    pub fn render(&self) -> Image {
        let mut res = Image::new(self.scene.cam.height, self.scene.cam.width);
        for x in 0..self.scene.cam.height {
            for y in 0..self.scene.cam.width {
                if let Some(c) = self.cast_ray(x, y) {
                    res.set(x, y, c)
                }
            }
        }
        res
    }

    fn cast_ray(&self, x: usize, y: usize) -> Option<Color> {
        let origin = self.scene.cam.get_pixel_pos(x, y);
        let direction = Vector::from(self.scene.cam.pos, origin);

        let ray = Ray {
            color: crate::common::WHITE,
            origin,
            direction,
        };

        // f64 is not hashable so we use u64 and convert f64 using to_bits
        let mut intersections = HashMap::<u64, Color>::new();

        for obj in self.scene.objects.iter() {
            if let Some(d) = obj.intersects(ray) {
                let (r, g, b) = obj.diffusion(crate::common::ORIGIN);
                let c = Color(r as u8, g as u8, b as u8);
                intersections.insert(d.to_bits(), c);
            }
        }

        if intersections.is_empty() {
            None
        } else {
            let min = intersections
                .keys()
                .map(|bits| f64::from_bits(bits.clone()))
                .fold(f64::MAX, |acc, x| if acc > x { x } else { acc });
            intersections.get(&min.to_bits()).map(|c| c.clone())
        }
    }
}
