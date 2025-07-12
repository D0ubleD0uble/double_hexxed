pub const SCALE: f64 = 0.25;
pub fn step_size() -> (f64, f64) {
    let step_x = (457. * SCALE) / (3.0_f64).sqrt();
    let step_y = (484. * SCALE) / 2.0;
    (step_x, step_y)
}
pub fn image_size() -> (f64, f64) {
    (
        (466. * SCALE * 2.0) / (3.0_f64).sqrt(),
        (554. * SCALE * 2.0) / 2.0,
    )
}
