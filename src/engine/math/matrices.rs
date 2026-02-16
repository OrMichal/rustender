use std::ops::{Add, Index, Mul, Sub};

/// struct representing a mathematical matrix of multiple values
///
/// # Type Parameters
/// - `T`: The type of objects Matrix contains. Must implement `Add`, `Sub`, `Mul` for matrix operations
/// - `R`: Number of rows in a matrix
/// - `C`: Number of columns in a matrix
///
/// # Examples
///```
///use rustender::engine::math::matrices::Matrix;
///
///let matrix = Matrix::<i32, 2, 3>([
/// [1, 2, 3],
/// [4, 5, 6]
///]);
///
///let matrix2: Matrix::<i32, 2, 3> = [
/// [1, 2, 3],
/// [4, 5, 6]
///].into();
///
///let matrix3 = Matrix::from([
/// [1, 2, 3],
/// [4, 5, 6]
///]);
///
///let matrix4 = Matrix([
/// [1, 2, 3],
/// [4, 5, 6]
///]);
///```
#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Matrix<T, const R: usize, const C: usize>(pub [[T; C]; R]);

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {

}

impl<T, const R: usize, const C: usize> From<[[T; C]; R]> for Matrix<T, R, C> {
    fn from(value: [[T; C]; R]) -> Self {
        Self(value)
    }
}

/// Indexing matrix using double usize tupple
/// # Examples
/// ```
/// use rustender::engine::math::matrices::Matrix;
///
/// let matrix = Matrix([
///     [1, 2, 3]
/// ]);
///
/// //returns 3
/// let value = matrix[(0, 2)];
/// ```
impl<T, const R: usize, const C: usize> Index<(usize, usize)> for Matrix<T, R, C> {
    type Output = T;

    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        &self.0[r][c]
    }
}

/// Adding two matrices of same type using + operator
///
/// # Examples
/// ```
/// use rustender::engine::math::matrices::Matrix;
///
/// let matrix1 = Matrix([
///     [1, 2, 3],
///     [4, 5, 6]
/// ]);
///
/// let matrix2 = Matrix([
///     [1, 1, 1],
///     [1, 1, 1]
/// ]);
///
/// //returns Matrix::<i32, 2, 3>([
/// //    [2, 3, 4],
/// //    [5, 6, 7]
/// //])
/// let result = matrix1 + matrix2;
/// ```
impl<T: Add<Output = T> + Copy, const R: usize, const C: usize> Add<Self> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    #[allow(clippy::needless_range_loop)]
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.0;
        for i in 0..R {
            for j in 0..C {
                result[i][j] = result[i][j] + rhs.0[i][j];
            }
        }

        Matrix(result)
    }
}

/// Subtracting two matrices of same type using - operator
///
/// # Examples
/// ```
/// use rustender::engine::math::matrices::Matrix;
///
/// let matrix1 = Matrix([
///     [1, 2, 3],
///     [4, 5, 6]
/// ]);
///
/// let matrix2 = Matrix([
///     [1, 1, 1],
///     [1, 1, 1]
/// ]);
///
/// //returns Matrix([
/// //    [0, 1, 2],
/// //    [3, 4, 5]
/// //])
/// let result = matrix1 - matrix2;
/// ```
impl<T: Sub<Output = T> + Copy, const R: usize, const C: usize> Sub<Self> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;
    #[allow(clippy::needless_range_loop)]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.0;
        for i in 0..R {
            for j in 0..C {
                result[i][j] = result[i][j] - rhs.0[i][j];
            }
        }

        Matrix(result)
    }
}

/// Multiplying of two matrices.
///
/// # Examples
/// ```
/// use rustender::engine::math::matrices::Matrix;
/// 
/// let matrix1 = Matrix([
///     [1, 2, 3]
/// ]);
///
/// let matrix2 = Matrix([
///     [1],
///     [2],
///     [3]
/// ]);
///
/// // returns Matrix([
/// //   [1],
/// //   [4],
/// //   [9]
/// //])
/// let result = matrix1 * matrix2;
///
/// ```
impl<T: Add<Output = T> + Mul<Output = T> + Copy + Default, const R: usize, const C: usize, const K: usize> Mul<Matrix<T, C, K>> for Matrix<T, R, C> {
    type Output = Matrix<T, R, K>;
    fn mul(self, rhs: Matrix<T, C, K>) -> Self::Output {
        let mut result = Matrix([[T::default(); K]; R]);

        for i in 0..R {
            for j in 0..K {
                let mut sum = T::default();

                for k in 0..C {
                    sum = sum + self.0[i][k] * rhs.0[k][j];
                }
                result.0[i][j] = sum;
            }
        }

        result
    }
}
