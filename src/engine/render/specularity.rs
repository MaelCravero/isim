use crate::common::*;

pub fn process(
    light_vector: NormalVector,
    light_intensity: (f64, f64, f64),
    reflection: f64,
    reflected: NormalVector,
) -> Color {
    // I = k * (S.L)^ns * I_l
    let (lr, lg, lb) = light_intensity;
    let mean_intensity = lr / 3.0 + lg / 3.0 + lb / 3.0;

    let ns = 3.0;
    let dot = NormalVector::dot_product(&light_vector, &reflected);
    if dot < 0.0 {
        return Color(0, 0, 0);
    }

    let i = (reflection * dot.powf(ns) * mean_intensity * (u8::MAX as f64)) as u8;

    Color(i, i, i)
}
