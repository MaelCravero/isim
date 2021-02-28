use crate::*;

pub fn get() -> scene::Scene {
    let cam = scene::Camera::new(
        Point(0.0, 0.0, 0.0),
        Point(0.0, 0.0, 8.0),
        Vector::new(1.0, 0.0, 0.0),
        90.0,
        90.0,
        2.0,
        400,
        400,
    );

    let mut objs = Vec::<Box<dyn scene::Object>>::new();
    let texture = scene::texture::UniformTexture::new(common::RED, 1.0, 1.0);
    objs.push(Box::new(
        scene::Sphere::<scene::texture::UniformTexture>::new(Point(0.0, 0.0, 6.0), 1.0, texture),
    ));
    let texture = scene::texture::UniformTexture::new(common::RED, 0.5, 0.5);
    objs.push(Box::new(
        scene::Sphere::<scene::texture::UniformTexture>::new(Point(3.0, -1.0, 6.0), 0.6, texture),
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
        scene::Sphere::<scene::texture::UniformTexture>::new(Point(-3.3, -3.3, 6.0), 1.0, texture),
    ));

    // "Skybox"
    let texture = scene::texture::UniformTexture::new(Color(135, 206, 235), 1.0, 0.0);
    objs.push(Box::new(
        scene::Sphere::<scene::texture::UniformTexture>::new(common::ORIGIN, 1000.0, texture),
    ));

    let mut lights = Vec::<Box<dyn scene::Light>>::new();
    //lights.push(Box::new(scene::light::PointLight::new(
    //Point(4.0, 4.0, 6.0),
    //(1.0, 1.0, 1.0),
    //)));

    lights.push(Box::new(scene::light::PointLight::new(
        Point(-4.0, -4.0, 0.0),
        (1.0, 1.0, 1.0),
    )));

    scene::Scene::new(cam, lights, objs)
}
