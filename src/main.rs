mod common;
mod engine;
mod geometry;
mod image;
mod scene;

use std::fs::File;
use std::path::Path;

use common::{Color, Point};
use geometry::Vector;
use image::Image;

fn save_image(path: &str, image: Image) {
    let path = Path::new(&path);
    let file = File::create(path);

    if let Ok(mut file) = file {
        match image.to_ppm(&mut file) {
            Ok(_) => println!("Success!"),
            _ => println!("Could not write: {}", path.display()),
        }
    } else {
        println!("Could not open: {}", path.display())
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let cam = scene::Camera::new(
        common::ORIGIN,
        Point(0.0, 0.0, 1.0),
        Vector::new(1.0, 0.0, 0.0),
        80.0,
        80.0,
        5.0,
        400,
        600,
    );

    let scene = scene::Scene::new(cam, Vec::<Box<dyn scene::Object>>::new());
    let image = engine::render(scene);

    save_image(&args[1], image)
}
