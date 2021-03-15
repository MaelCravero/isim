use crate::{
    common::{Point, ORIGIN},
    geometry::Vector,
};

pub struct Camera {
    pub pos: Point,
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
        up: Vector,
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
        let vunit_y =
            Vector::cross_product(&up, &Vector::from(center_of_view, pos)).normalize() * unit_y;
        let vunit_x = up * unit_x;

        let vec_center = Vector::from(ORIGIN, center_of_view).normalize() * z_min;

        let top_left =
            vec_center + vunit_y * (width as f64 / 2.0) + vunit_x * (height as f64 / 2.0);

        Camera {
            pos,
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
}
