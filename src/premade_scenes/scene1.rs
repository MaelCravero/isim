use crate::*;

pub fn get(res_x: usize, res_y: usize) -> scene::Scene {
    let cam = scene::Camera::new(
        Point(0.0, 0.0, 0.0),
        Point(0.0, 0.0, 8.0),
        Vector::new(1.0, 0.0, 0.0).normalize(),
        45.0,
        45.0,
        2.0,
        res_x,
        res_y,
    );

    let objs: scene::ObjectContainer = vec![
        Box::new(sphere! {(0.0, 0.0, 16.0); 1.0; <uniform>(common::RED, 1.0, 1.0)}),
        Box::new(sphere! {(3.0, -1.0, 16.0); 0.6; <uniform>(common::RED, 0.5, 0.5)}),
        Box::new(sphere! {(0.0, 0.0, 19.0); 4.0; <uniform>(common::GREEN, 1.0, 1.0)}),
        Box::new(sphere! {(1.0, 1.0, 16.3); 0.7; <uniform>(common::BLUE, 1.0, 1.3)}),
        Box::new(sphere! {(-3.3, -3.3, 16.0); 1.0; <uniform>(common::WHITE, 1.0, 1.0)}),
        // Skybox
        //Box::new(sphere! {common::ORIGIN; 1000.0; <uniform>(Color(135, 206, 235), 1.0, 0.0)}),
    ];

    let mut lights = scene::LightContainer::new();
    //lights.push(Box::new(scene::light::PointLight::new(
    //Point(4.0, 4.0, 16.0),
    //(1.0, 1.0, 1.0),
    //)));

    //lights.push(Box::new(scene::light::PointLight::new(
    //Point(0.0, 0.0, 0.0),
    //(1.0, 1.0, 1.0),
    //)));

    lights.push(Box::new(scene::light::PointLight::new(
        Point(-5.0, -5.0, 11.0),
        (1.0, 1.0, 1.0),
    )));

    scene::Scene::new(cam, lights, objs)
}
