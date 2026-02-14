use crate::engine::math::Matrix;

/// Matrix for rotating point in 3D space
pub fn rotation_matrix_x_3d(angle: f64) -> Matrix<f64, 3, 3> {
    Matrix([
        [1.0, 0.0, 0.0],
        [0.0, angle.cos(), -angle.sin()],
        [0.0, angle.sin(), angle.cos()]
    ])
}

pub fn rotation_matrix_y_3d(angle: f64) -> Matrix<f64, 3, 3> {
    Matrix([
        [angle.cos(), 0.0, angle.sin()],
        [0.0, 1.0, 0.0],
        [-angle.sin(), 0.0, angle.cos()]
    ])
}

pub fn rotation_matrix_z_3d(angle: f64) -> Matrix<f64, 3, 3> {
    Matrix([
        [angle.cos(), -angle.sin(), 0.0],
        [angle.sin(), angle.cos(), 0.0],
        [0.0, 0.0, 1.0]
    ])
}

/// Matrix for rotating points in 2D space counter clockwise
pub fn rotation_matrix_2d(angle: f64) -> Matrix<f64, 2, 2> {
    Matrix([
        [angle.cos(), angle.sin()],
        [-angle.sin(), angle.cos()]
    ])
}
