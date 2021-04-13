mod common;
mod engine;
mod geometry;
mod image;
mod lsystem;
mod premade_scenes;
mod scene;

use std::fs::File;
use std::path::Path;

use common::{Color, Point};
use geometry::Vector;
use image::Image;

fn save_image(path: &str, image: &Image) {
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

fn save_gif(path: &str, image: &Vec<Image>) {
    let path = Path::new(&path);
    let file = File::create(path);

    if let Ok(mut file) = file {
        match Image::save_as_gif(image, &mut file) {
            Ok(_) => println!("Success!"),
            _ => println!("Could not write: {}", path.display()),
        }
    } else {
        println!("Could not open: {}", path.display())
    }
}

fn generate_multiple_plants(args: &Vec<String>) -> crate::scene::ObjectContainer {
    let mut objs: scene::ObjectContainer = vec![Box::new(
        triangle! {Point(-10.0, 2000.0, 2000.0), Point(-10.0, -2000.0, 2000.0), Point(-10.0, -2000.0, -2000.0); <uniform>(Color(124,252,0), 1.0, 1.0)},
    )];

    objs.append({
        let lsystem = lsystem::LSystem::from_file(&args[2]).unwrap().generate();

        &mut lsystem.translate(
            Point(-10.0, 0.0, 20.0),
            //Point(-10.0, 10.0, 30.0),
            Vector::new(1.0, 0.0, 0.0).normalize(),
            Vector::new(0.0, -1.0, 0.0).normalize(),
            //Vector::new(1.0, -1.0, -1.0).normalize(),
            0.5,
            0.5,
        )
    });

    objs.append({
        let lsystem = lsystem::LSystem::from_file(&args[2]).unwrap().generate();

        &mut lsystem.translate(
            Point(-10.0, 4.0, 25.0),
            //Point(-10.0, 10.0, 30.0),
            Vector::new(1.0, 0.0, 0.0).normalize(),
            Vector::new(0.0, -1.0, 0.0).normalize(),
            //Vector::new(1.0, -1.0, -1.0).normalize(),
            0.4,
            0.1,
        )
    });

    objs.append({
        let lsystem = lsystem::LSystem::from_file(&args[2]).unwrap().generate();

        &mut lsystem.translate(
            Point(-10.0, -8.0, 24.0),
            //Point(-10.0, 10.0, 30.0),
            Vector::new(1.0, 0.0, 0.0).normalize(),
            Vector::new(0.0, -1.0, 0.0).normalize(),
            //Vector::new(1.0, -1.0, -1.0).normalize(),
            0.3,
            0.1,
        )
    });

    objs.append({
        let lsystem = lsystem::LSystem::from_file(&args[2]).unwrap().generate();

        &mut lsystem.translate(
            Point(-10.0, -8.0, 16.0),
            //Point(-10.0, 10.0, 30.0),
            Vector::new(1.0, 0.0, 0.0).normalize(),
            Vector::new(0.0, -1.0, 0.0).normalize(),
            //Vector::new(1.0, -1.0, -1.0).normalize(),
            0.3,
            0.1,
        )
    });

    objs.append({
        let lsystem = lsystem::LSystem::from_file(&args[2]).unwrap().generate();

        &mut lsystem.translate(
            Point(-10.0, 3.0, 21.0),
            //Point(-10.0, 10.0, 30.0),
            Vector::new(1.0, 0.0, 0.0).normalize(),
            Vector::new(0.0, -1.0, 0.0).normalize(),
            //Vector::new(1.0, -1.0, -1.0).normalize(),
            0.4,
            0.1,
        )
    });

    objs.append({
        let lsystem = lsystem::LSystem::from_file(&args[2]).unwrap().generate();

        &mut lsystem.translate(
            Point(-10.0, 3.0, 32.0),
            //Point(-10.0, 10.0, 30.0),
            Vector::new(1.0, 0.0, 0.0).normalize(),
            Vector::new(0.0, -1.0, 0.0).normalize(),
            //Vector::new(1.0, -1.0, -1.0).normalize(),
            0.4,
            0.1,
        )
    });

    objs
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let (res_x, res_y) = (200, 200);
    let cam = scene::Camera::new(
        Point(0.0, 0.0, 0.0),
        Point(0.0, 0.0, 20.0),
        Vector::new(1.0, 0.0, 0.0).normalize(),
        90.0,
        90.0,
        1.0,
        res_x,
        res_y,
    );
    let mut lights = scene::LightContainer::new();
    let mut objs: scene::ObjectContainer = vec![
        //Box::new(sphere! {(0.0, 1.5, 11.0); 1.0; <uniform>(common::RED, 1.0, 1.0)}),
        //Box::new(
        //triangle! {Point(0.0, 0.0, 6.0), Point(0.0, -1.0, 6.0), Point(-1.0, 1.0, 6.0);
        //<uniform>(common::BLUE, 1.0, 1.0)},
        //),
        //Box::new(
        //cylinder! {Point(0.0, 1.5, 11.0), Point(4.0, 5.5, 11.0); 0.7;
        //<uniform>(common::GREEN, 1.0, 1.0)},
        //),
        //Box::new(cylinder! {Point(0.0, 0.0, 4.0), Point(0.0, 0.0, 16.0); 0.3;
        //<uniform>(common::YELLOW, 1.0, 1.0)}),
        //Box::new(sphere! {Point(0.0, 0.0, 4.0); 0.3;
        //<uniform>(common::YELLOW, 1.0, 1.0)}),
        Box::new(
            triangle! {Point(-10.0, 2000.0, 2000.0), Point(-10.0, -2000.0, 2000.0), Point(-10.0, -2000.0, -2000.0); <uniform>(Color(124,252,0), 1.0, 1.0)},
        ),
    ];

    objs.append({
        let lsystem = lsystem::LSystem::from_file(&args[2]).unwrap().generate();

        &mut lsystem.translate(
            Point(-10.0, 0.0, 20.0),
            //Point(-10.0, 10.0, 30.0),
            Vector::new(1.0, 0.0, 0.0).normalize(),
            Vector::new(0.0, -1.0, 0.0).normalize(),
            //Vector::new(1.0, -1.0, -1.0).normalize(),
            0.5,
            0.1,
        )
    });

    lights.push(Box::new(scene::light::PointLight::new(
        Point(0.0, -2.0, 0.0),
        (1.0, 1.0, 1.0),
    )));

    let scene = scene::Scene::new(cam, lights, objs);

    let mut engine = engine::Engine::new(scene);
    //let mut engine = engine::Engine::new(premade_scenes::scene1::get(res_x, res_y));
    //let mut engine = engine::Engine::new(premade_scenes::scene2::get(res_x, res_y));

    engine.set_diffuse();
    engine.set_specular();
    engine.set_ambient((0.1, 0.1, 0.12));
    //engine.set_reflection();
    //engine.set_intersect();

    //let image = engine.render();

    let res = engine.travelling(
        &mut |c| {
            c.move_to(Point(0.0, 0.0, 10.0));
        },
        2,
    );

    save_gif(&args[1], &res);
}
