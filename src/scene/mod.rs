mod camera;
mod cylinder;
pub mod light;
mod ray;
mod scene;
mod sphere;
pub mod texture;
mod triangle;

use crate::{common::Point, geometry::NormalVector};

pub use camera::Camera;
pub use ray::Ray;

pub use cylinder::Cylinder;
pub use sphere::Sphere;
pub use triangle::Triangle;

pub use scene::LightContainer;
pub use scene::LightType;
pub use scene::ObjectContainer;
pub use scene::ObjectType;
pub use scene::Scene;

pub trait TextureMaterial {
    fn diffusion(&self, u: f64, v: f64) -> (f64, f64, f64);
    fn specularity(&self, x: usize, y: usize) -> f64;
}

pub trait Object {
    fn intersects(&self, ray: Ray) -> Option<f64>;
    fn normal(&self, p: Point) -> NormalVector;
    fn diffusion(&self, p: Point) -> (f64, f64, f64);
    fn specularity(&self, p: Point) -> f64;
    fn map_to_texture(&self, p: Point) -> (f64, f64);
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

    (($x:expr, $y:expr, $z:expr); $r:expr; <uvmapped>($c:expr, $d:expr, $s:expr)) => {{
        use crate::common::Point;
        use crate::scene::texture::UVMapTexture;
        use crate::scene::Sphere;

        Sphere::<UVMapTexture>::new(Point($x, $y, $z), $r, UVMapTexture::new($c, $d, $s))
    }};
}

#[macro_export]
macro_rules! triangle {
    ($a:expr, $b:expr, $c:expr; <uniform>($color:expr, $d:expr, $s:expr)) => {{
        use crate::common::Point;
        use crate::scene::texture::UniformTexture;
        use crate::scene::Triangle;

        Triangle::<UniformTexture>::new(($a, $b, $c), UniformTexture::new($color, $d, $s))
    }};
}

#[macro_export]
macro_rules! cylinder {
    ($a:expr, $b:expr; $r:expr; <uniform>($c:expr, $d:expr, $s:expr)) => {{
        use crate::common::Point;
        use crate::scene::texture::UniformTexture;
        use crate::scene::Cylinder;

        Cylinder::<UniformTexture>::new($a, $b, $r, UniformTexture::new($c, $d, $s))
    }};
}
