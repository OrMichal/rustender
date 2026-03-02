use crate::{GDI, Vec2, graphics::Vec3};

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
    pub vertices: [Vec3; 3],
}

impl Triangle {
    /// Constructor of a triangle
    ///
    /// # Parameters
    /// - `vertices`: Array of 3 3D vectors
    pub fn new(vertices: [Vec3; 3]) -> Self {
        Self { vertices }
    }

    pub fn rasterize(&self, focal_length: f64) -> Vec<Vec2> {
        let mut points = Vec::new();
        let vertices = self.get_projected_vertices(focal_length);

        let min_x = vertices.iter().map(|v| v.x).fold(f64::INFINITY, f64::min).floor() as i32;
        let max_x = vertices.iter().map(|v| v.x).fold(f64::NEG_INFINITY, f64::max).ceil() as i32;
        let min_y = vertices.iter().map(|v| v.y).fold(f64::INFINITY, f64::min).floor() as i32;
        let max_y = vertices.iter().map(|v| v.y).fold(f64::NEG_INFINITY, f64::max).ceil() as i32;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec2 {
                    x: x as f64,
                    y: y as f64,
                };

                let w0 = Self::edge(vertices[1], vertices[2], p);
                let w1 = Self::edge(vertices[2], vertices[0], p);
                let w2 = Self::edge(vertices[0], vertices[1], p);

                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    points.push(p);
                }
            }
        }

        points
    }

    pub fn get_projected_vertices(&self, focal_length: f64) -> [Vec2; 3] {
        [
            self.vertices[0].get_projected_2d(focal_length),
            self.vertices[1].get_projected_2d(focal_length),
            self.vertices[2].get_projected_2d(focal_length),
        ]
    }

    fn edge(a: Vec2, b: Vec2, c: Vec2) -> f64 {
        (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
    }

    pub fn normal(&self) -> Vec3 {
        let line1 = self.vertices[1] - self.vertices[0];
        let line2 = self.vertices[2] - self.vertices[0];

        line1.cross(line2).normalize()
    }

    /// Rasterizes a 2D triangle and returns all pixels that should be filled
    /// Uses a scanline algorithm to fill the triangle
    ///
    /// # Parameters
    /// - `vertices`: Array of 3 Vec2 vertices representing the triangle corners
    ///
    /// # Returns
    /// Vector of Vec2 points representing all pixels inside the triangle
    pub fn rasterize_2d_triangle(vertices: [Vec2; 3]) -> Vec<Vec2> {
        let mut pixels = Vec::new();

        // Sort vertices by y-coordinate (top to bottom)
        let mut verts = vertices.to_vec();
        verts.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

        let v0 = verts[0];
        let v1 = verts[1];
        let v2 = verts[2];

        // Draw the upper triangle (v0 to v1)
        if v1.y != v0.y {
            let steps = (v1.y - v0.y).abs().ceil() as i32;
            for i in 0..=steps {
                let t = i as f64 / steps as f64;
                let left_x = v0.x + (v2.x - v0.x) * t;
                let right_x = v0.x + (v1.x - v0.x) * t;
                let y = v0.y + t * (v1.y - v0.y);

                Self::fill_scanline(&mut pixels, left_x.min(right_x), right_x.max(left_x), y);
            }
        }

        // Draw the lower triangle (v1 to v2)
        if v2.y != v1.y {
            let steps = (v2.y - v1.y).abs().ceil() as i32;
            for i in 0..=steps {
                let t = i as f64 / steps as f64;
                let left_x = v1.x + (v2.x - v1.x) * t;
                let right_x = v0.x + (v2.x - v0.x) * ((v1.y - v0.y + t * (v2.y - v1.y)) / (v2.y - v0.y));
                let y = v1.y + t * (v2.y - v1.y);

                Self::fill_scanline(&mut pixels, left_x.min(right_x), right_x.max(left_x), y);
            }
        }

        pixels
    }

    /// Helper function to fill a horizontal scanline between two x coordinates
    fn fill_scanline(pixels: &mut Vec<Vec2>, x_start: f64, x_end: f64, y: f64) {
        let x_start_int = x_start.ceil() as i32;
        let x_end_int = x_end.floor() as i32;
        let y_int = y.round() as i32;

        for x in x_start_int..=x_end_int {
            pixels.push(Vec2::new(x as f64, y_int as f64));
        }
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
            [value[2][0], value[2][1], value[2][2]].into(),
        ])
    }
}
