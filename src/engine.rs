use std::collections::HashMap;

use crate::{
    common::Color,
    common::Point,
    geometry::Vector,
    image::Image,
    scene::{Object, Ray, Scene},
};

enum RenderingMode {
    Intersect,
    Diffuse,
}

pub struct Engine {
    scene: Scene,
    mode: RenderingMode,
}

impl Engine {
    pub fn new(scene: Scene) -> Engine {
        let mode = RenderingMode::Diffuse;
        Engine { scene, mode }
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

    fn process_hit(diffusion: (f64, f64, f64)) -> Color {
        let (r, g, b) = diffusion;
        Color(r as u8, g as u8, b as u8)
    }

    fn process_diffusion(
        light_vector: Vector,
        light_intensity: (f64, f64, f64),
        diffusion: (f64, f64, f64),
        normal: Vector,
    ) -> Color {
        // I = k * (N.L) * I_l
        let (kr, kg, kb) = diffusion;
        let (lr, lg, lb) = light_intensity;

        let proportion = Vector::dot_product(&normal, &light_vector);

        if proportion < 0.0 {
            return crate::common::BLACK;
        }

        let ir = kr * proportion * lr;
        let ig = kg * proportion * lg;
        let ib = kb * proportion * lb;

        Color(ir as u8, ig as u8, ib as u8)
    }

    fn process_point(&self, pos: Point, obj: &Box<dyn Object>) -> Color {
        let mut c = crate::common::BLACK;
        let normal = obj.normal(pos);

        for light in self.scene.lights.iter() {
            let light_vector = Vector::from(pos, light.pos()); // FIXME
            c += match self.mode {
                RenderingMode::Intersect => Engine::process_hit(obj.diffusion(pos)),
                RenderingMode::Diffuse => Engine::process_diffusion(
                    light_vector,
                    light.intensity(),
                    obj.diffusion(pos),
                    normal,
                ),
            };
        }

        c
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
        let mut intersections = HashMap::<u64, &Box<dyn Object>>::new();

        for obj in self.scene.objects.iter() {
            if let Some(d) = obj.intersects(ray) {
                intersections.insert(d.to_bits(), &obj);
            }
        }

        if intersections.is_empty() {
            return None;
        }

        let min = intersections
            .keys()
            .map(|bits| f64::from_bits(bits.clone()))
            .fold(f64::MAX, |acc, x| if acc > x { x } else { acc });
        let intersection_point = Vector::from(crate::common::ORIGIN, origin) + direction * min;
        let intersection_point = Point(
            intersection_point.x,
            intersection_point.y,
            intersection_point.z,
        );

        let closest: &Box<dyn Object> = intersections.get(&min.to_bits()).unwrap();

        Some(self.process_point(intersection_point, closest))
    }
}
