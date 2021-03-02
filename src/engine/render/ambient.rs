use crate::common::*;

pub fn process(
    light_intensity: (f64, f64, f64),
    ambient_light: (f64, f64, f64),
    diffusion: (f64, f64, f64),
) -> Color {
    let (lr, lg, lb) = light_intensity;
    let (ar, ag, ab) = ambient_light;
    let (r, g, b) = diffusion;
    let (kr, kg, kb) = (r as f64, g as f64, b as f64);

    Color(
        (lr * ar * kr) as u8,
        (lg * ag * kg) as u8,
        (lb * ab * kb) as u8,
    )
}
