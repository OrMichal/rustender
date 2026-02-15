use crate::prelude::*;

///Struct representing an object for rendering
///
///# Examples
///```
///use rustender::prelude::*;
///
///let mesh = Mesh::new()
///   .set_triangles(vec![
///       [
///           Vec3::from([1.0, 1.0, 1.0]),
///           Vec3::from([2.0, 2.0, 2.0]),
///           Vec3::from([3.0, 3.0, 3.0])
///       ].into()
///   ])
///   .set_rotation(0.0, 0.0, 0.0)
///   .set_rotation_x(0.0)
///   .build();
///```
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    /// Vec of Triangle structs representing triangles which is object made of.
    pub triangles: Vec<Triangle>,
    /// Position of a Mesh represented by a 3D Vector
    pub position: Vec3,
    /// XYZ rotation of a Mesh
    pub rotation: Rotation,
}

impl Mesh {
    /// Begins the construction of a Mesh using builder patttern.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> MeshBuilder {
        MeshBuilder {
            triangles: vec![],
            position: None,
            rotation: None,
        }
    }
}

/// Struct used in a process of building a Mesh.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MeshBuilder {
    pub triangles: Vec<Triangle>,
    pub position: Option<Vec3>,
    pub rotation: Option<Rotation>,
}

impl MeshBuilder {
    /// Finalizes Mesh building process.
    ///
    /// # Returns
    /// finished and complete build of Mesh  struct
    pub fn build(self) -> Mesh {
        let triangles = self.triangles;
        let position = self.position.unwrap_or([0.0, 0.0, 0.0].into());
        let rotation = self.rotation.unwrap_or(Rotation::new(0.0, 0.0, 0.0));

        Mesh { triangles, position, rotation }
    }

    /// Sets triangle array on Mesh
    ///
    /// # Parameters
    /// -`triangles`: triangle Vec for visual representation of a Mesh
    ///
    /// # Returns
    /// itself for continuous building chain
    pub fn set_triangles(mut self, triangles: Vec<Triangle>) -> Self {
        self.triangles = triangles;

        self
    }

    /// Adds triangle to a Vec of triangles for visual representation of a Mesh
    ///
    /// # Parameters
    /// -`triangle`: triangle struct to be added to triangles in Mesh
    ///
    /// # Returns
    /// itself for continuous building chain
    ///
    pub fn add_triangle(mut self, triangle: Triangle) -> Self {
        self.triangles.push(triangle);

        self
    }

    /// Sets rotation angles for each axis
    ///
    /// # Parameters
    /// -`x`: angle on the X axis in radians
    /// -`y`: angle on the Y axis in radians
    /// -`z`: angle on the Z axis in radians
    ///
    /// # Returns
    /// itself for continuous building chain
    ///
    pub fn set_rotation(mut self, x: f64, y: f64, z: f64) -> Self {
        self.rotation = Some(Rotation::new(x, y, z));

        self
    }

    /// Sets rotation on X axis
    ///
    /// # Parameters
    /// -`angle`: angle value to be set for Meshes X axis rotation in radians
    ///
    /// # Returns
    /// itself for continuous building chain
    ///
    pub fn set_rotation_x(mut self, angle: f64) -> Self {
        match &mut self.rotation {
            Some(r) => r.x = angle,
            None => {
                self.rotation = Some(Rotation::new(angle, 0.0, 0.0));
            }
        }

        self
    }

    /// Sets rotation on Y axis
    ///
    /// # Parameters
    /// -`angle`: angle value to be set for Meshes Y axis rotation in radians
    ///
    /// # Returns
    /// itself for continuous building chain
    ///
    pub fn set_rotation_y(mut self, angle: f64) -> Self {
        match &mut self.rotation {
            Some(r) => r.y = angle,
            None => {
                self.rotation = Some(Rotation::new(0.0, angle, 0.0));
            }
        }

        self
    }


    /// Sets rotation on Z axis
    ///
    /// # Parameters
    /// -`angle`: angle value to be set for Meshes Z axis rotation in radians
    ///
    /// # Returns
    /// itself for continuous building chain
    ///
    pub fn set_rotation_z(mut self, angle: f64) -> Self {
        match &mut self.rotation {
            Some(r) => r.z = angle,
            None => {
                self.rotation = Some(Rotation::new(0.0, 0.0, angle));
            }
        }

        self
    }

    /// Sets 3D location of a Mesh
    ///
    /// # Parameters
    /// -`position`: location of a Mesh to be set represented by a 3D vector
    ///
    /// # Returns
    /// itself for continuous building chain
    ///
    pub fn set_position(mut self, position: Vec3) -> Self {
        self.position = Some(position);

        self
    }

    /// Sets X axis location of a Mesh
    ///
    /// # Parameters
    /// -`x`: angle to be set for the Meshes X axis
    ///
    /// # Returns
    /// itself for continuous building chain
    ///
    pub fn set_position_x(mut self, x: f64) -> Self {
        match &mut self.position {
            Some(p) => p.x = x,
            None => {
                self.position = Some([x, 0.0, 0.0].into())
            }
        }

        self
    }

    /// Sets Y axis location of a Mesh
    ///
    /// # Parameters
    /// -`y`: angle to be set for the Meshes Y axis
    ///
    /// # Returns
    /// itself for continuous building chain
    ///
    pub fn set_position_y(mut self, y: f64) -> Self {
        match &mut self.position {
            Some(p) => p.y = y,
            None => {
                self.position = Some([0.0, y, 0.0].into())
            }
        }

        self
    }


    /// Sets Z axis location of a Mesh
    ///
    /// # Parameters
    /// -`z`: angle to be set for the Meshes Z axis
    ///
    /// # Returns
    /// itself for continuous building chain
    ///
    pub fn set_position_z(mut self, z: f64) -> Self {
        match &mut self.position {
            Some(p) => p.z = z,
            None => {
                self.position = Some([0.0, 0.0, z].into())
            }
        }

        self
    }
}
