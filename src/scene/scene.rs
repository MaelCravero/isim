use std::collections::HashSet;

use super::{camera::Camera, Object};

pub struct Scene {
    pub cam: Camera,
    //lights: HashSet<>, TODO
    pub objects: HashSet<Box<dyn Object>>,
}

impl Scene {
    pub fn new(cam: Camera, objects: HashSet<Box<dyn Object>>) -> Scene {
        Scene { cam, objects }
    }
}
