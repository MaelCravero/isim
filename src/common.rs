#[derive(Copy, Clone, Debug)]
pub struct Color(pub u8, pub u8, pub u8);

pub const BLACK: Color = Color(0, 0, 0);
pub const WHITE: Color = Color(255, 255, 255);
pub const RED: Color = Color(255, 0, 0);
pub const GREEN: Color = Color(0, 255, 0);
pub const BLUE: Color = Color(0, 0, 255);

#[derive(Copy, Clone, Debug)]
pub struct Point(pub f64, pub f64, pub f64);

pub const ORIGIN: Point = Point(0.0, 0.0, 0.0);
