use crate::{common::*, geometry::Vector};

pub struct Camera {
    pub pos: Point,
    center_of_view: Point,
    up: NormalVector,
    x_fov_angle: f64,
    y_fov_angle: f64,
    pub z_min: f64,
    pub height: usize,
    pub width: usize,
    top_left: Vector,
    vunit_x: Vector,
    vunit_y: Vector,
}

impl Camera {
    pub fn new(
        pos: Point,
        center_of_view: Point,
        up: NormalVector,
        x_fov_angle: f64,
        y_fov_angle: f64,
        z_min: f64,
        height: usize,
        width: usize,
    ) -> Self {
        let w = 2.0 * z_min * (y_fov_angle / 2.0).to_radians().tan();
        let h = 2.0 * z_min * (x_fov_angle / 2.0).to_radians().tan();

        let unit_y = w / (width as f64);
        let unit_x = h / (height as f64);

        let vec_center = Vector::from(pos, center_of_view).normalize().vector() * z_min;

        let vunit_y = Vector::cross_product(&up.vector(), &vec_center.normalize().vector())
            .normalize()
            .vector()
            * unit_y;
        let vunit_x = up.vector() * unit_x;

        let top_left = Vector::from(ORIGIN, pos)
            + vec_center
            + vunit_y * (width as f64 / 2.0)
            + vunit_x * (height as f64 / 2.0);

        Camera {
            pos,
            center_of_view,
            up,
            x_fov_angle,
            y_fov_angle,
            z_min,
            height,
            width,
            top_left,
            vunit_x,
            vunit_y,
        }
    }

    pub fn get_pixel_pos(&self, x: usize, y: usize) -> Point {
        let pos = self.top_left - self.vunit_x * (x as f64) - self.vunit_y * (y as f64);

        Point(pos.x, pos.y, pos.z)
    }

    pub fn move_to(&mut self, pos: Point) {
        *self = Camera::new(
            pos,
            self.center_of_view,
            self.up,
            self.x_fov_angle,
            self.y_fov_angle,
            self.z_min,
            self.height,
            self.width,
        );
    }
}
