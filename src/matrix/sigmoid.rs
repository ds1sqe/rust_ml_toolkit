pub fn sigmoid_f32(x: f32) -> f32 {
    return 1.0 / (1.0 + (-x).exp());
}
pub fn sigmoid_f64(x: f64) -> f64 {
    return 1.0 / (1.0 + (-x).exp());
}
