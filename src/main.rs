mod common;
mod engine;
mod geometry;
mod image;
mod scene;
mod scene1;

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

    let (res_x, res_y) = (1200, 1200);
    let cam = scene::Camera::new(
        Point(0.0, 0.0, 0.0),
        Point(0.0, 0.0, 4.0),
        Vector::new(1.0, 0.0, 0.0),
        90.0,
        90.0,
        2.0,
        res_x,
        res_y,
    );
    let mut lights = Vec::<Box<dyn scene::Light>>::new();
    let mut objs = Vec::<Box<dyn scene::Object>>::new();

    let texture = scene::texture::UniformTexture::new(common::RED, 1.0, 1.0);
    objs.push(Box::new(
        scene::Sphere::<scene::texture::UniformTexture>::new(Point(0.0, 1.5, 10.0), 1.0, texture),
    ));

    let texture = scene::texture::UniformTexture::new(common::GREEN, 1.0, 1.0);
    objs.push(Box::new(
        scene::Sphere::<scene::texture::UniformTexture>::new(Point(0.0, -1.5, 10.0), 1.0, texture),
    ));

    //let texture = scene::texture::UniformTexture::new(common::GREEN, 1.0, 1.0);
    //objs.push(Box::new(
    //scene::Sphere::<scene::texture::UniformTexture>::new(Point(0.0, 0.0, 4.0), 1.0, texture),
    //));

    lights.push(Box::new(scene::light::PointLight::new(
        Point(0.0, 0.0, 4.0),
        (1.0, 1.0, 1.0),
    )));

    let scene = scene::Scene::new(cam, lights, objs);

    let mut engine = engine::Engine::new(scene);
    //let mut engine = engine::Engine::new(scene1::get(res_x, res_y));

    engine.set_diffuse();
    engine.set_specular();
    engine.set_ambient((0.1, 0.1, 0.12));
    engine.set_reflection();
    //engine.set_intersect();

    let image = engine.render();

    save_image(&args[1], image)
}
