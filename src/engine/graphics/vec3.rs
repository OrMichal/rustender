use crate::{Vec2};
use std::{ops::{Add, Sub}};

/// Struct representing a mathematical 3D vector
///
/// # Examples
/// ```
/// use rustender::prelude::*;
///
/// let vec1 = Vec3::new(1.0, 2.0, 3.0);
///
/// let vec2: Vec3 = [1.3, 2.1, 4.5].into();
///
/// let vec3 = Vec3::from([1.7, 2.4, 6.1]);
///
/// ```
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    /// X axis location of 3D vector
    pub x: f64,
    /// Y axis location of 3D vector
    pub y: f64,
    /// Z axis location of 3D vector
    pub z: f64,
}

impl Vec3 {
    /// Constructor function to create a Vec3 from floating point numbers array
    ///
    /// # Examples
    /// ```
    /// use rustender::prelude::*;
    /// let vec1 = Vec3::new(1.0, 2.1, 3.2);
    /// ```
    ///
    /// # Parameters
    /// - `x`: X axis location of a 3D vector
    /// - `y`: Y axis location of a 3D vector
    /// - `z`: Z axis location of a 3D vector
    ///
    /// # Returns
    /// new Vec3 struct
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn get_projected_2d(&self, focal_length: f64) -> Vec2 {
        Vec2::new((self.x * focal_length) / self.z, (self.y * focal_length) / self.z)
    }

    pub fn dot(self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        [
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x
        ].into()
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        let len = self.length();

        [
            self.x / len,
            self.y / len,
            self.z / len
        ].into()
    }
}

/// Implementation of From trait for flexibility when constructing a Vec3
///
/// # Examples
///
/// ```
/// use rustender::prelude::*;
///
/// let vec1: Vec3 = [1.0, 2.0, 3.0].into();
///
/// let vec2 = Vec3::from([1.0, 2.0, 3.0]);
/// ```
impl From<[f64; 3]> for Vec3 {
    /// # Parameters
    /// - `value`: Array of 3 64 bit floating point numbers to represent direction of a Vec3
    ///
    /// # Returns
    /// new Vec3 struct
    fn from(value: [f64; 3]) -> Self {
        Self { x: value[0], y: value[1], z: value[2] }
    }
}

/// Implementation of + addition operator
///
/// # Examples
///
/// ```
/// use rustender::prelude::*;
///
/// let vec1: Vec3 = [1.0, 1.0, 2.0].into();
///
/// let vec2: Vec3 = [0.7, 0.5, 1.0].into();
/// 
/// // returns new Vec3 (1.7, 1.5, 3.0)
/// let result = vec1 + vec2;
/// ```
impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        [
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        ].into()
    }
}

/// Implementation of - subtraction operator
///
/// # Examples
///
/// ```
/// use rustender::prelude::*;
///
/// let vec1: Vec3 = [1.0, 1.0, 2.0].into();
///
/// let vec2: Vec3 = [0.7, 0.5, 1.0].into();
/// 
/// // returns new Vec3 (0.3, 0.5, 1.0)
/// let result = vec1 - vec2;
/// ```
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        [
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z
        ].into()
    }
}
