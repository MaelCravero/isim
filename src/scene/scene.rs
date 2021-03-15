use super::{Camera, Light, Object};

pub type LightType = Box<dyn Light>;
pub type ObjectType = Box<dyn Object>;

pub type LightContainer = Vec<LightType>;
pub type ObjectContainer = Vec<ObjectType>;

pub struct Scene {
    pub cam: Camera,
    pub lights: LightContainer,
    pub objects: ObjectContainer,
}

impl Scene {
    pub fn new(cam: Camera, lights: LightContainer, objects: ObjectContainer) -> Scene {
        Scene {
            cam,
            lights,
            objects,
        }
    }
}
