use crate::graphics::Vec3;

/// Triangle struct for representing a triangle in computer graphics. Used for creating Meshes
///
/// # Parameters
/// - `vertices`: Array of 3 3D vectors representing points of the triangle
///
/// # Example
/// ```
/// use rustender::prelude::*;
///
/// let vec1: Vec3 = [1.0, 2.0, 3.0].into();
/// let vec2: Vec3 = [1.2, 2.2, 3.2].into();
/// let vec3: Vec3 = [1.4, 2.4, 3.4].into();
///
/// let tri1 = Triangle::new([vec1.clone(), vec2.clone(), vec3.clone()]);
///
/// let tri2: Triangle = [vec1.clone(), vec2.clone(), vec3.clone()].into();
///
/// let tri3: Triangle = [
///     [1.0, 2.0, 3.0],
///     [1.2, 2.2, 3.2],
///     [1.4, 2.4, 3.4]
/// ].into();
/// ```
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    /// Array of 3 3D vectors representing corners of triangle
    pub vertices: [Vec3; 3]
}

impl Triangle {
    /// Constructor of a triangle
    ///
    /// # Parameters
    /// - `vertices`: Array of 3 3D vectors
    pub fn new(vertices: [Vec3; 3]) -> Self {
        Self { vertices }
    }
}

/// Implementation of From trait giving user a flexible way of constructing a triangle
///
/// # Example
/// ```
/// use rustender::prelude::*;
///
/// let vec1: Vec3 = [1.0, 2.0, 3.0].into();
/// let vec2: Vec3 = [1.2, 2.2, 3.2].into();
/// let vec3: Vec3 = [1.4, 2.4, 3.4].into();
///
/// let tri: Triangle = [vec1, vec2, vec3].into();
/// 
/// ```
impl From<[Vec3; 3]> for Triangle {
    fn from(value: [Vec3; 3]) -> Self {
        Triangle::new(value)
    }
}

/// Implementation of From trait giving user a flexible way of constructing a triangle
///
/// # Example
/// ```
/// use rustender::prelude::*;
///
/// let tri: Triangle = [
///     [1.0, 2.0, 3.0],
///     [1.5, 2.5, 6.0],
///     [6.1, 7.1, 8.1]
/// ].into();
/// 
/// ```
impl From<[[f64; 3]; 3]> for Triangle {
    fn from(value: [[f64; 3]; 3]) -> Self {
        Triangle::new([
            [value[0][0], value[0][1], value[0][2]].into(),
            [value[1][0], value[1][1], value[1][2]].into(),
            [value[2][0], value[2][1], value[2][2]].into()
        ])
    }
}
