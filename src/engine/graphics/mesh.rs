use crate::prelude::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub position: Vec3,
    pub rotation: Rotation,
    pub scale: Vec3
}

impl Mesh {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> MeshBuilder {
        MeshBuilder {
            triangles: vec![],
            position: None,
            rotation: None,
            scale: None
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MeshBuilder {
    pub triangles: Vec<Triangle>,
    pub position: Option<Vec3>,
    pub rotation: Option<Rotation>,
    pub scale: Option<Vec3>
}

impl MeshBuilder {
    pub fn build(self) -> Mesh {
        let triangles = self.triangles;
        let position = self.position.unwrap_or([0.0, 0.0, 0.0].into());
        let rotation = self.rotation.unwrap_or(Rotation::new(0.0, 0.0, 0.0));
        let scale = self.scale.unwrap_or([1.0, 1.0, 1.0].into());

        Mesh { triangles, position, rotation, scale }
    }

    pub fn set_triangles(mut self, triangles: Vec<Triangle>) -> Self {
        self.triangles = triangles;

        self
    }

    pub fn add_triangle(mut self, triangle: Triangle) -> Self {
        self.triangles.push(triangle);

        self
    }

    pub fn set_rotation(mut self, x: f64, y: f64, z: f64) -> Self {
        self.rotation = Some(Rotation::new(x, y, z));

        self
    }

    pub fn set_rotation_x(mut self, angle: f64) -> Self {
        match &mut self.rotation {
            Some(r) => r.x = angle,
            None => {
                self.rotation = Some(Rotation::new(angle, 0.0, 0.0));
            }
        }

        self
    }

    pub fn set_rotation_y(mut self, angle: f64) -> Self {
        match &mut self.rotation {
            Some(r) => r.y = angle,
            None => {
                self.rotation = Some(Rotation::new(0.0, angle, 0.0));
            }
        }

        self
    }


    pub fn set_rotation_z(mut self, angle: f64) -> Self {
        match &mut self.rotation {
            Some(r) => r.z = angle,
            None => {
                self.rotation = Some(Rotation::new(0.0, 0.0, angle));
            }
        }

        self
    }

    pub fn set_position(mut self, position: Vec3) -> Self {
        self.position = Some(position);

        self
    }

    pub fn set_position_x(mut self, x: f64) -> Self {
        match &mut self.position {
            Some(p) => p.x = x,
            None => {
                self.position = Some([x, 0.0, 0.0].into())
            }
        }

        self
    }


    pub fn set_position_y(mut self, y: f64) -> Self {
        match &mut self.position {
            Some(p) => p.y = y,
            None => {
                self.position = Some([0.0, y, 0.0].into())
            }
        }

        self
    }


    pub fn set_position_z(mut self, z: f64) -> Self {
        match &mut self.position {
            Some(p) => p.z = z,
            None => {
                self.position = Some([0.0, 0.0, z].into())
            }
        }

        self
    }

    pub fn set_scale(mut self, scale: Vec3) -> Self {
        self.scale = Some(scale);

        self
    }

    pub fn set_scale_x(mut self, x: f64) -> Self {
        match &mut self.scale {
            Some(s) => s.x = x,
            None => {
                self.scale = Some([x, 0.0, 0.0].into());
            }
        }

        self
    }


    pub fn set_scale_y(mut self, y: f64) -> Self {
        match &mut self.scale {
            Some(s) => s.y = y,
            None => {
                self.scale = Some([0.0, y, 0.0].into());
            }
        }

        self
    }


    pub fn set_scale_z(mut self, z: f64) -> Self {
        match &mut self.scale {
            Some(s) => s.z = z,
            None => {
                self.scale = Some([0.0, 0.0, z].into());
            }
        }

        self
    }
}
