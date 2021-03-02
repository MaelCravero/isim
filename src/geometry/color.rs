#[derive(Copy, Clone, Debug)]
pub struct Color(pub u8, pub u8, pub u8);

fn add_color(a: u8, b: u8) -> u8 {
    match a.overflowing_add(b) {
        (v, false) => v,
        (_, true) => u8::MAX,
    }
}

impl std::ops::Add for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color(
            add_color(self.0, rhs.0),
            add_color(self.1, rhs.1),
            add_color(self.2, rhs.2),
        )
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        *self = Color(
            add_color(self.0, rhs.0),
            add_color(self.1, rhs.1),
            add_color(self.2, rhs.2),
        )
    }
}
