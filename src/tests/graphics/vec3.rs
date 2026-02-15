#[allow(unused_imports)]
use crate::prelude::*;

#[test]
pub fn vec3_init() {
    let vec1 = Vec3::new(1.0, 2.0, 3.0);

    let vec2: Vec3 = [1.0, 2.0, 3.0].into();

    let vec3 = Vec3::from([1.0, 2.0, 3.0]);

    assert_eq!(vec1, [1.0, 2.0, 3.0].into());
    assert_eq!(vec2, [1.0, 2.0, 3.0].into());
    assert_eq!(vec3, [1.0, 2.0, 3.0].into());
}

#[test]
pub fn vec3_add() {
    let vec1: Vec3 = [1.0, 2.0, 3.0].into();
    let vec2: Vec3 = [1.0, 2.0, 3.0].into();

    let result = vec1 + vec2;

    assert_eq!(result, [2.0, 4.0, 6.0].into());
}

#[test]
pub fn vec3_sub() {
    
    let vec1: Vec3 = [1.0, 2.0, 3.0].into();
    let vec2: Vec3 = [1.0, 2.0, 3.0].into();

    let result = vec1 - vec2;

    assert_eq!(result, [0.0, 0.0, 0.0].into());
}
