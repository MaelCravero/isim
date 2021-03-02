use crate::common::*;

pub fn process(diffusion: (f64, f64, f64)) -> Color {
    let (r, g, b) = diffusion;
    Color(r as u8, g as u8, b as u8)
}
