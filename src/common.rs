#[derive(Copy, Clone, Debug)]
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Copy, Clone, Debug)]
pub struct Point(pub f64, pub f64, pub f64);

pub const ORIGIN: Point = Point(0.0, 0.0, 0.0);
