use super::{Camera, Light, Object};

pub struct Scene {
    pub cam: Camera,
    pub lights: Vec<Box<dyn Light>>,
    pub objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn new(cam: Camera, lights: Vec<Box<dyn Light>>, objects: Vec<Box<dyn Object>>) -> Scene {
        Scene {
            cam,
            lights,
            objects,
        }
    }
}
