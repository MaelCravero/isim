use crate::{
    common::{Point, ORIGIN},
    geometry::Vector,
};

pub struct Camera {
    pub pos: Point,
    center_of_view: Point,
    up: Vector,
    x_fov_angle: f64,
    y_fov_angle: f64,
    pub z_min: f64,
    pub height: usize,
    pub width: usize,
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
        Camera {
            pos,
            center_of_view,
            up,
            x_fov_angle,
            y_fov_angle,
            z_min,
            height,
            width,
        }
    }

    pub fn get_pixel_pos(&self, x: usize, y: usize) -> Point {
        let w = 2.0 * self.z_min * (self.y_fov_angle / 2.0).to_radians().tan();
        let h = 2.0 * self.z_min * (self.x_fov_angle / 2.0).to_radians().tan();

        let unit_y = w / (self.width as f64);
        let unit_x = h / (self.height as f64);
        let vunit_y =
            Vector::cross_product(&self.up, &Vector::from(self.center_of_view, self.pos))
                .normalize()
                * unit_y;
        let vunit_x = self.up * unit_x;

        let vec_center = Vector::from(ORIGIN, self.center_of_view).normalize() * self.z_min;

        let top_left = vec_center
            + vunit_y * (self.width as f64 / 2.0)
            + vunit_x * (self.height as f64 / 2.0);

        let pos = top_left - vunit_x * (x as f64) - vunit_y * (y as f64);

        Point(pos.x, pos.y, pos.z)
    }
}
