#[allow(unused_imports)]
use crate::engine::math::matrices::Matrix;

#[test]
pub fn matrix_add() {
    let matrix1 = Matrix::<i32, 2, 3>([
        [1, 2, 3],
        [4, 5, 6]
    ]);

    let matrix2 = Matrix::<i32, 2, 3>([
        [1, 1, 1],
        [1, 1, 1]
    ]);

    let result = matrix1 + matrix2;

    assert_eq!(result, Matrix::<i32, 2, 3>([
            [2, 3, 4],
            [5, 6, 7]
    ]));
}

#[test]
pub fn matrix_sub() {
    let matrix1 = Matrix([
        [1, 2, 3],
        [4, 5, 6]
    ]);

    let matrix2 = Matrix([
        [1, 1, 1],
        [1, 1, 1]
    ]);

    let result = matrix1 - matrix2;

    assert_eq!(result, Matrix([
            [0, 1, 2],
            [3, 4, 5]
    ]));
}

#[test]
pub fn matrix_mul() {
    let matrix1 = Matrix([
        [1, 2, 3],
        [4, 5, 6]
    ]);

    let matrix2 = Matrix([
        [7, 8],
        [9, 10],
        [11, 12]
    ]);

    let result = matrix1 * matrix2;

    assert_eq!(result, Matrix([
            [58, 64],
            [139, 154]
    ]));
}
