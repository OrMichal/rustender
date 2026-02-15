#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Rotation {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Rotation {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}
