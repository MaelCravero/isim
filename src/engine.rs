use crate::{common::Color, geometry::Vector, image::Image, scene::Scene};

struct Engine {
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
                res.set(x, y, self.cast_ray(x, y))
            }
        }
        res
    }

    fn cast_ray(&self, x: usize, y: usize) -> Color {
        crate::common::BLACK
    }
}
