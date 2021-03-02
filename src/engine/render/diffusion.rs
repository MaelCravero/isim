use crate::common::*;

pub fn process(
    light_vector: Vector,
    light_intensity: (f64, f64, f64),
    diffusion: (f64, f64, f64),
    normal: Vector,
) -> Color {
    // I = k * (N.L) * I_l
    let (kr, kg, kb) = diffusion;
    let (lr, lg, lb) = light_intensity;

    let proportion = Vector::dot_product(&normal, &light_vector.normalize());

    if proportion < 0.0 {
        return crate::common::BLACK;
    }

    let ir = kr * proportion * lr;
    let ig = kg * proportion * lg;
    let ib = kb * proportion * lb;

    Color(ir as u8, ig as u8, ib as u8)
}
