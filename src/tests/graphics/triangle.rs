#[allow(unused_imports)]
use crate::prelude::*;

#[test]
pub fn triangle_init() {
    let vec1: Vec3 = [1.0, 1.1, 1.1].into();
    let vec2: Vec3 = [2.0, 2.1, 2.1].into();
    let vec3: Vec3 = [3.0, 3.1, 3.1].into();

    let tri1: Triangle = [vec1, vec2, vec3].into();

    assert_eq!(tri1, [
        [1.0, 1.1, 1.1],
        [2.0, 2.1, 2.1],
        [3.0, 3.1, 3.1]
    ].into())
}
