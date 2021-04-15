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

const GIF_SPEED: usize = 2;

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
        match Image::save_as_gif(image, &mut file, GIF_SPEED) {
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

        &mut lsystem
            .translate(
                Point(-10.0, 0.0, 20.0),
                //Point(-10.0, 10.0, 30.0),
                Vector::new(1.0, 0.0, 0.0).normalize(),
                Vector::new(0.0, -1.0, 0.0).normalize(),
                //Vector::new(1.0, -1.0, -1.0).normalize(),
                0.5,
            )
            .last_mut()
            .unwrap()
    });

    objs.append({
        let lsystem = lsystem::LSystem::from_file(&args[2]).unwrap().generate();

        &mut lsystem
            .translate(
                Point(-10.0, 4.0, 25.0),
                //Point(-10.0, 10.0, 30.0),
                Vector::new(1.0, 0.0, 0.0).normalize(),
                Vector::new(0.0, -1.0, 0.0).normalize(),
                //Vector::new(1.0, -1.0, -1.0).normalize(),
                0.4,
            )
            .last_mut()
            .unwrap()
    });

    objs.append({
        let lsystem = lsystem::LSystem::from_file(&args[2]).unwrap().generate();

        &mut lsystem
            .translate(
                Point(-10.0, -8.0, 24.0),
                //Point(-10.0, 10.0, 30.0),
                Vector::new(1.0, 0.0, 0.0).normalize(),
                Vector::new(0.0, -1.0, 0.0).normalize(),
                //Vector::new(1.0, -1.0, -1.0).normalize(),
                0.3,
            )
            .last_mut()
            .unwrap()
    });

    objs.append({
        let lsystem = lsystem::LSystem::from_file(&args[2]).unwrap().generate();

        &mut lsystem
            .translate(
                Point(-10.0, -8.0, 16.0),
                //Point(-10.0, 10.0, 30.0),
                Vector::new(1.0, 0.0, 0.0).normalize(),
                Vector::new(0.0, -1.0, 0.0).normalize(),
                //Vector::new(1.0, -1.0, -1.0).normalize(),
                0.3,
            )
            .last_mut()
            .unwrap()
    });

    objs.append({
        let lsystem = lsystem::LSystem::from_file(&args[2]).unwrap().generate();

        &mut lsystem
            .translate(
                Point(-10.0, 3.0, 21.0),
                //Point(-10.0, 10.0, 30.0),
                Vector::new(1.0, 0.0, 0.0).normalize(),
                Vector::new(0.0, -1.0, 0.0).normalize(),
                //Vector::new(1.0, -1.0, -1.0).normalize(),
                0.4,
            )
            .last_mut()
            .unwrap()
    });

    objs.append({
        let lsystem = lsystem::LSystem::from_file(&args[2]).unwrap().generate();

        &mut lsystem
            .translate(
                Point(-10.0, 3.0, 32.0),
                //Point(-10.0, 10.0, 30.0),
                Vector::new(1.0, 0.0, 0.0).normalize(),
                Vector::new(0.0, -1.0, 0.0).normalize(),
                //Vector::new(1.0, -1.0, -1.0).normalize(),
                0.4,
            )
            .last_mut()
            .unwrap()
    });

    objs
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let (res_x, res_y) = (900, 900);
    let cam = scene::Camera::new(
        Point(4.0, 0.0, -1.0),
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
        Box::new(
            triangle! {Point(-10.0, 10.0, 30.0), Point(-10.0, 10.0, 10.0), Point(-10.0, -10.0, 30.0);
            <uniform>(Color(166,166,166), 1.0, 1.0)},
        ),
        Box::new(
            triangle! {Point(-10.0, -10.0, 10.0), Point(-10.0, 10.0, 10.0), Point(-10.0, -10.0, 30.0);
            <uniform>(Color(166,166,166), 1.0, 1.0)},
        ),
    ];

    /*
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
        )[args[3].parse::<usize>().unwrap()]
    });
    */
    let plants = {
        let lsystem = lsystem::LSystem::from_file(&args[2]).unwrap().generate();

        &mut lsystem.translate(
            Point(-10.0, 0.0, 20.0),
            //Point(-10.0, 10.0, 30.0),
            Vector::new(1.0, 0.0, 0.0).normalize(),
            Vector::new(0.0, -1.0, 0.0).normalize(),
            //Vector::new(1.0, -1.0, -1.0).normalize(),
            0.5,
        )
    };

    lights.push(Box::new(scene::light::PointLight::new(
        Point(6.0, -15.0, 12.0),
        (1.0, 1.0, 1.0),
    )));
    //lights.push(Box::new(scene::light::PointLight::new(
    //Point(6.0, 5.0, 12.0),
    //(1.0, 1.0, 1.0),
    //)));

    let is_gif = args[1].contains("gif");
    let is_growth = args[1].contains("growth");

    let scene = scene::Scene::new(
        cam,
        lights,
        if is_growth {
            objs
        } else {
            plants.pop().unwrap()
        },
    );

    let mut engine = engine::Engine::new(scene);
    //let mut engine = engine::Engine::new(premade_scenes::scene1::get(res_x, res_y));
    //let mut engine = engine::Engine::new(premade_scenes::scene2::get(res_x, res_y));

    engine.set_diffuse();
    engine.set_specular();
    engine.set_ambient((0.4, 0.4, 0.4));
    //engine.set_reflection();
    //engine.set_intersect();

    if is_gif {
        if is_growth {
            println!("Rendering growth");
            let res = engine.render_growth(
                &mut |c| c.rotate_around_center_of_view(10.0f64.to_radians()),
                //&mut |_| (),
                9,
                plants,
            );
            save_gif(&args[1], &res);
        } else {
            println!("Rendering travelling");
            let res = engine.travelling(
                &mut |c| c.rotate_around_center_of_view(10.0f64.to_radians()),
                36,
            );
            save_gif(&args[1], &res);
        }
    } else {
        println!("Rendering image");
        let image = engine.render();

        save_image(&args[1], &image);
    }
}
