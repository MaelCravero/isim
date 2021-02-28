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
    Specular,
}

pub struct Engine {
    scene: Scene,
    mode: Vec<RenderingMode>,
}

impl Engine {
    pub fn new(scene: Scene) -> Engine {
        Engine {
            scene,
            mode: Vec::new(),
        }
    }

    pub fn reset_mode(&mut self) -> &mut Self {
        self.mode = Vec::new();
        self
    }

    pub fn set_intersect(&mut self) -> &mut Self {
        self.mode.push(RenderingMode::Intersect);
        self
    }

    pub fn set_diffuse(&mut self) -> &mut Self {
        self.mode.push(RenderingMode::Diffuse);
        self
    }

    pub fn set_specular(&mut self) -> &mut Self {
        self.mode.push(RenderingMode::Specular);
        self
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

    fn process_reflection(
        light_vector: Vector,
        light_intensity: (f64, f64, f64),
        reflection: f64,
        reflected: Vector,
    ) -> Color {
        // I = k * (S.L)^ns * I_l
        let (lr, lg, lb) = light_intensity;
        let mean_intensity = lr / 3.0 + lg / 3.0 + lb / 3.0;

        let ns = 1.8;
        let i = (reflection
            * Vector::dot_product(&light_vector, &reflected).powf(ns)
            * mean_intensity) as u8;
        //* (u8::MAX as f64)) as u8;

        Color(i, i, i)
    }

    fn process_point(&self, pos: Point, obj: &Box<dyn Object>, ray: &Ray) -> Color {
        let mut c = crate::common::BLACK;
        let normal = obj.normal(pos);

        let reflected =
            ray.direction - normal * 2.0 * (Vector::dot_product(&normal, &ray.direction));

        for light in self.scene.lights.iter() {
            let light_vector = Vector::from(pos, light.pos()); // FIXME
            for mode in self.mode.iter() {
                c += match mode {
                    RenderingMode::Intersect => Engine::process_hit(obj.diffusion(pos)),
                    RenderingMode::Diffuse => Engine::process_diffusion(
                        light_vector,
                        light.intensity(),
                        obj.diffusion(pos),
                        normal,
                    ),
                    RenderingMode::Specular => Engine::process_reflection(
                        light_vector,
                        light.intensity(),
                        obj.specularity(pos),
                        reflected,
                    ),
                };
            }
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

        Some(self.process_point(intersection_point, closest, &ray))
    }
}