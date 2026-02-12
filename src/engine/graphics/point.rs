#[allow(dead_code)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
    shading: u8
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64, shading: u8) -> Self {
        Self { x, y, z, shading }
    }
}
