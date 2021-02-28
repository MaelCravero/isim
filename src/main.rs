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
        90.0,
        90.0,
        5.0,
        400,
        400,
    );

    let mut objs = Vec::<Box<dyn scene::Object>>::new();
    let texture = scene::texture::UniformTexture::new(common::RED, 1.0, 1.0);
    objs.push(Box::new(
        scene::Sphere::<scene::texture::UniformTexture>::new(Point(0.0, 0.0, 6.0), 1.0, texture),
    ));
    let texture = scene::texture::UniformTexture::new(common::GREEN, 1.0, 1.0);
    objs.push(Box::new(
        scene::Sphere::<scene::texture::UniformTexture>::new(Point(0.0, 0.0, 8.0), 4.0, texture),
    ));
    let texture = scene::texture::UniformTexture::new(common::BLUE, 1.0, 1.0);
    objs.push(Box::new(
        scene::Sphere::<scene::texture::UniformTexture>::new(Point(1.0, 1.0, 7.0), 1.0, texture),
    ));
    let texture = scene::texture::UniformTexture::new(common::WHITE, 1.0, 1.0);
    objs.push(Box::new(
        scene::Sphere::<scene::texture::UniformTexture>::new(Point(-3.0, -3.0, 6.0), 1.0, texture),
    ));

    // "Skybox"
    let texture = scene::texture::UniformTexture::new(Color(135, 206, 235), 1.0, 1.0);
    objs.push(Box::new(
        scene::Sphere::<scene::texture::UniformTexture>::new(common::ORIGIN, 1000.0, texture),
    ));

    let mut lights = Vec::<Box<dyn scene::Light>>::new();
    lights.push(Box::new(scene::light::PointLight::new(
        Point(4.0, 4.0, 6.0),
        (1.0, 1.0, 1.0),
    )));

    let scene = scene::Scene::new(cam, lights, objs);
    let engine = engine::Engine::new(scene);
    let image = engine.render();

    save_image(&args[1], image)
}
