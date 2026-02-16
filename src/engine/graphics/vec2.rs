use std::ops::{Add, Sub};

#[allow(unused_imports)]
use crate::prelude::*;

/// 2D mathematical vector representation
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    /// X axis location of the vector
    pub x: f64,
    /// Y axis location of the vector
    pub y: f64
}

#[allow(dead_code)]
impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Add<Self> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub<Self> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}
