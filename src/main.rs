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

    let (res_x, res_y) = (700, 700);
    let cam = scene::Camera::new(
        Point(0.0, 0.0, 0.0),
        Point(0.0, 0.0, 4.0),
        Vector::new(1.0, 0.0, 0.0),
        90.0,
        90.0,
        1.0,
        res_x,
        res_y,
    );
    let mut lights = scene::LightContainer::new();
    let mut objs: scene::ObjectContainer = vec![
        Box::new(sphere! {(0.0, 1.5, 11.0); 1.0; <uniform>(common::RED, 1.0, 1.0)}),
        Box::new(sphere! {(0.0, -0.5, 9.0); 0.5; <uniform>(common::GREEN, 1.0, 1.0)}),
        // Aligned
        //Box::new(sphere! {(0.0, -3.0, 8.0); 1.0; <uniform>(common::RED, 1.0, 1.0)}),
        //Box::new(sphere! {(0.0, 3.0, 8.0); 2.0; <uniform>(common::GREEN, 1.0, 1.0)}),
        //Box::new(sphere! {(0.0, 0.0, 18.0); 2.0; <uniform>(common::GREEN, 1.0, 1.0)}),
        //Box::new(sphere! {(2.0, 0.0, 14.9); 0.2; <uniform>(common::RED, 1.0, 1.0)}),
        //Box::new(sphere! {(0.0, -2.0, 14.9); 0.2; <uniform>(common::RED, 1.0, 1.0)}),
        //Box::new(sphere! {(0.0, 2.0, 14.9); 0.2; <uniform>(common::RED, 1.0, 1.0)}),
    ];

    objs.push(Box::new(scene::Triangle::new(
        (
            Point(10.0, 0.0, 16.0),
            Point(0.0, -10.0, 15.0),
            Point(-10.0, 10.0, 12.0),
        ),
        scene::texture::UniformTexture::new(common::BLUE, 1.0, 0.5),
    )));

    lights.push(Box::new(scene::light::PointLight::new(
        Point(-2.0, 1.0, 6.0),
        (1.0, 1.0, 1.0),
    )));

    let scene = scene::Scene::new(cam, lights, objs);

    let mut engine = engine::Engine::new(scene);
    //let mut engine = engine::Engine::new(scene1::get(res_x, res_y));

    engine.set_diffuse();
    engine.set_specular();
    engine.set_ambient((0.1, 0.1, 0.12));
    //engine.set_reflection();
    //engine.set_intersect();

    let image = engine.render();

    save_image(&args[1], image)
}
