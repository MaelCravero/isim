mod camera;
pub mod light;
mod ray;
mod scene;
mod sphere;
pub mod texture;

use crate::{common::Point, geometry::Vector};

pub use camera::Camera;
pub use ray::Ray;
pub use sphere::Sphere;

pub use scene::LightContainer;
pub use scene::LightType;
pub use scene::ObjectContainer;
pub use scene::ObjectType;
pub use scene::Scene;

pub trait TextureMaterial {
    fn diffusion(&self, p: Point) -> (f64, f64, f64);
    fn specularity(&self, p: Point) -> f64;
}

pub trait Object {
    fn intersects(&self, ray: Ray) -> Option<f64>;
    fn normal(&self, p: Point) -> Vector;
    fn diffusion(&self, p: Point) -> (f64, f64, f64);
    fn specularity(&self, p: Point) -> f64;
}

pub trait Light {
    fn pos(&self) -> Point;
    fn intensity(&self) -> (f64, f64, f64);
}

#[macro_export]
macro_rules! sphere {
    (($x:expr, $y:expr, $z:expr); $r:expr; <uniform>($c:expr, $d:expr, $s:expr)) => {{
        use crate::common::Point;
        use crate::scene::texture::UniformTexture;
        use crate::scene::Sphere;

        Sphere::<UniformTexture>::new(Point($x, $y, $z), $r, UniformTexture::new($c, $d, $s))
    }};

    ($p:expr; $r:expr; <uniform>($c:expr, $d:expr, $s:expr)) => {{
        use crate::scene::texture::UniformTexture;
        use crate::scene::Sphere;

        Sphere::<UniformTexture>::new($p, $r, UniformTexture::new($c, $d, $s))
    }};
}
