use super::TextureMaterial;
use crate::common::{Color, Point};
use imagelib::{DynamicImage, ImageBuffer, Rgb};

pub struct UniformTexture {
    color: Color,
    diff: f64,
    refl: f64,
}

impl UniformTexture {
    pub fn new(color: Color, diff: f64, refl: f64) -> UniformTexture {
        UniformTexture { color, diff, refl }
    }
}

impl TextureMaterial for UniformTexture {
    fn diffusion(&self, _u: f64, _v: f64) -> (f64, f64, f64) {
        let Color(r, g, b) = self.color;
        let (r, g, b) = (r as f64, g as f64, b as f64);
        (self.diff * r, self.diff * g, self.diff * b)
    }

    fn specularity(&self, _x: usize, _y: usize) -> f64 {
        self.refl
    }
}

pub struct UVMapTexture {
    buffer: imagelib::RgbImage,
    diff: f64,
    refl: f64,
}

impl UVMapTexture {
    pub fn new(name: String, diff: f64, refl: f64) -> UVMapTexture {
        let buffer = imagelib::open(name).unwrap().to_rgb8();
        UVMapTexture { buffer, diff, refl }
    }
}

impl TextureMaterial for UVMapTexture {
    fn diffusion(&self, u: f64, v: f64) -> (f64, f64, f64) {
        let mut i = (1.0 - u) * self.buffer.width() as f64;
        let mut j = v * self.buffer.height() as f64;

        if i < 0.0 {
            i = 0.0
        }
        if j < 0.0 {
            j = 0.0
        }

        if i > self.buffer.width() as f64 - 1.0 {
            i = self.buffer.width() as f64 - 1.0;
        }
        if j > self.buffer.height() as f64 - 1.0 {
            j = self.buffer.height() as f64 - 1.0;
        }

        let pixel = self.buffer.get_pixel(i as u32, j as u32);
        let r = pixel.0[0] as f64;
        let g = pixel.0[1] as f64;
        let b = pixel.0[2] as f64;
        (r, g, b)
    }

    fn specularity(&self, _x: usize, _y: usize) -> f64 {
        self.refl
    }
}
