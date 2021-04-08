use crate::*;

pub fn get(res_x: usize, res_y: usize) -> scene::Scene {
    let cam = scene::Camera::new(
        Point(0.0, 0.0, 0.0),
        Point(0.0, 0.0, 8.0),
        Vector::new(1.0, 0.0, 0.0),
        90.0,
        90.0,
        2.0,
        res_x,
        res_y,
    );

    let mut lights = scene::LightContainer::new();
    let objs: scene::ObjectContainer = vec![
        Box::new(sphere! {(0.0, 1.5, 11.0); 1.0; <uniform>(common::RED, 1.0, 1.0)}),
        Box::new(sphere! {(0.0, -0.5, 9.0); 0.5; <uniform>(common::GREEN, 1.0, 1.0)}),
        Box::new(sphere! {(0.0, -1.5, 5.0); 0.2; <uniform>(common::GREEN, 1.0, 1.0)}),
        // Background triangle
        Box::new(
            triangle! {Point(10.0, 0.0, 16.0), Point(0.0, -10.0, 15.0), Point(-10.0, 10.0, 12.0);
            <uniform>(common::BLUE, 1.0, 0.5)},
        ),
        // Yellow foreground structure
        Box::new(
            triangle! {Point(1.0, 0.0, 6.0), Point(0.0, -1.0, 8.0), Point(-1.0, 0.0, 6.0);
            <uniform>(common::YELLOW, 1.0, 1.0)},
        ),
        Box::new(
            triangle! {Point(1.0, 0.0, 6.0), Point(0.0, 1.0, 8.0), Point(-1.0, 0.0, 6.0);
            <uniform>(common::YELLOW, 1.0, 1.0)},
        ),
        Box::new(
            triangle! {Point(1.0, 0.0, 6.0), Point(0.0, -1.0, 8.0), Point(2.0, -1.0, 7.0);
            <uniform>(common::YELLOW, 1.0, 1.0)},
        ),
    ];

    lights.push(Box::new(scene::light::PointLight::new(
        Point(0.0, -2.0, 0.0),
        (1.0, 1.0, 1.0),
    )));

    scene::Scene::new(cam, lights, objs)
}
