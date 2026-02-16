use crate::prelude::*;

/// Struct for representing camera and FOV in rendering process
///
/// # Example
/// ```
/// use rustender::prelude::*;
///
/// let camera = Camera::new()
///     .location([1.0, 0.0, 0.0].into())
///     .width(80)
///     .height(50)
///     .build();
/// ```
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    /// Location of a camera view
    location: Vec3,
    /// Width of a camera view
    width: usize,
    /// Height of a camera view
    height: usize,
    /// Rotation of a camera view by X, Y and Z axis by radians
    rotation: Rotation,
    /// Field of view of the camera
    fov: f64
}

impl Camera {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> CameraBuilder {
        CameraBuilder {
            location: None,
            width: None,
            height: None,
            fov: None
        }
    }

    pub fn get_focal_length(&self) -> f64 {
        (self.height / 2) as f64 / (self.fov / 2.0).tan()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct CameraBuilder {
    location: Option<Vec3>,
    width: Option<usize>,
    height: Option<usize>,
    fov: Option<f64>
}

impl CameraBuilder {
    pub fn build(self) -> Camera {
        let location = self.location.unwrap_or([0.0, 0.0, 0.0].into());
        let width = self.width.unwrap_or(100);
        let height = self.height.unwrap_or(60);
        let fov = self.fov.unwrap_or(90.0_f64);

        Camera { location, width, height, rotation: Rotation::new(0.0, 0.0, 0.0), fov }
    }

    pub fn location(mut self, location: Vec3) -> Self {
        self.location = Some(location);

        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);

        self
    }

    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);

        self
    }

    pub fn fov(mut self, fov: f64) -> Self {
        self.fov = Some(fov);

        self
    }
}
