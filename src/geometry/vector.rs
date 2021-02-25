use std::ops;

use crate::common::Point;

/// Standard 3D Vector
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Standard vector constructors
impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn from(src: Point, dst: Point) -> Vector {
        Vector {
            x: dst.0 - src.0,
            y: dst.1 - src.1,
            z: dst.2 - src.2,
        }
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Vector {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, v: f64) -> Vector {
        Vector::new(v * self.x, v * self.y, v * self.z)
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;
    fn div(self, v: f64) -> Vector {
        Vector::new(self.x / v, self.y / v, self.z / v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = v1 + v1;
        assert_eq!(Vector::new(2.0, 4.0, 6.0), v2)
    }

    #[test]
    fn test_sub() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = v1 - v1;
        assert_eq!(Vector::new(0.0, 0.0, 0.0), v2)
    }

    #[test]
    fn test_mul() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = v1 * 2.0;
        assert_eq!(Vector::new(2.0, 4.0, 6.0), v2)
    }

    #[test]
    fn test_div() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = v1 / 2.0;
        assert_eq!(Vector::new(0.5, 1.0, 1.5), v2)
    }
}
