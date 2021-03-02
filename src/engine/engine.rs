use std::collections::HashMap;

use crate::{
    common::*,
    image::Image,
    scene::{Object, Ray, Scene},
};

use super::render::*;

enum RenderingMode {
    Intersect,
    Diffuse,
    Specular,
    Ambient((f64, f64, f64)),
    Reflection,
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

    pub fn set_ambient(&mut self, val: (f64, f64, f64)) -> &mut Self {
        self.mode.push(RenderingMode::Ambient(val));
        self
    }

    pub fn set_reflection(&mut self) -> &mut Self {
        self.mode.push(RenderingMode::Reflection);
        self
    }

    pub fn render(&self) -> Image {
        let mut res = Image::new(self.scene.cam.height, self.scene.cam.width);
        for x in 0..self.scene.cam.height {
            for y in 0..self.scene.cam.width {
                let origin = self.scene.cam.get_pixel_pos(x, y);
                let direction = Vector::from(self.scene.cam.pos, origin).normalize();

                let ray = Ray {
                    energy: 1.0,
                    origin,
                    direction,
                };

                if let Some(c) = self.cast_ray(ray) {
                    res.set(x, y, c)
                }
            }
        }
        res
    }

    fn process_point(&self, pos: Point, obj: &Box<dyn Object>, ray: &Ray, distance: f64) -> Color {
        let mut c = crate::common::BLACK;
        let normal = obj.normal(pos);

        let reflected = (ray.direction
            - normal * 2.0 * (Vector::dot_product(&normal, &ray.direction)))
        .normalize();

        let epsilon = 0.05;
        let epsilon_pos = (Vector::from(ORIGIN, pos) + reflected.normalize() * epsilon).to_point();

        for light in self.scene.lights.iter() {
            let mut in_shadow = false;
            for obj in self.scene.objects.iter() {
                if let Some(d) = obj.intersects(ray.clone()) {
                    if d < distance {
                        in_shadow = true;
                        break;
                    }
                }
            }
            if in_shadow {
                continue;
            }

            let light_vector = Vector::from(pos, light.pos());
            for mode in self.mode.iter() {
                c += match mode {
                    RenderingMode::Intersect => intersection::process(obj.diffusion(pos)),
                    RenderingMode::Diffuse => diffusion::process(
                        light_vector,
                        light.intensity(),
                        obj.diffusion(pos),
                        normal,
                    ),
                    RenderingMode::Specular => specularity::process(
                        light_vector,
                        light.intensity(),
                        obj.specularity(pos),
                        reflected,
                    ),
                    RenderingMode::Ambient(ambient_light) => {
                        ambient::process(light.intensity(), *ambient_light, obj.diffusion(pos))
                    }
                    RenderingMode::Reflection => {
                        let loss = 0.7;
                        let energy = ray.energy - loss;

                        if energy > 0.0 {
                            let reflection_ray = Ray {
                                energy,
                                origin: epsilon_pos,
                                direction: reflected,
                            };
                            if let Some(color) = self.cast_ray(reflection_ray) {
                                color
                            } else {
                                crate::common::BLACK
                            }
                        } else {
                            crate::common::BLACK
                        }
                    }
                };
            }
        }

        c
    }

    pub fn cast_ray(&self, ray: Ray) -> Option<Color> {
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
        let intersection_point =
            (Vector::from(crate::common::ORIGIN, ray.origin) + ray.direction * min).to_point();

        let closest: &Box<dyn Object> = intersections.get(&min.to_bits()).unwrap();

        Some(self.process_point(intersection_point, closest, &ray, min))
    }
}
