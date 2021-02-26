use super::{Camera, Object};

pub struct Scene {
    pub cam: Camera,
    //lights: HashSet<>, TODO
    pub objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn new(cam: Camera, objects: Vec<Box<dyn Object>>) -> Scene {
        Scene { cam, objects }
    }
}
