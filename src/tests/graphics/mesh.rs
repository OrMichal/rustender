#[allow(unused_imports)]
use crate::prelude::*;

#[test]
pub fn mesh_init() {
    let mesh = Mesh::new()
        .set_triangles(vec![
            [
                [1.0, 1.0, 1.0],
                [2.0, 2.0, 2.0],
                [3.0, 3.0, 3.0]
            ].into()
        ])
        .set_rotation(0.0, 0.0, 0.0)
        .set_rotation_x(2.0)
        .build();

    let expected = Mesh {
        triangles: vec![
            [
                [1.0, 1.0, 1.0],
                [2.0, 2.0, 2.0],
                [3.0, 3.0, 3.0]
            ].into()
        ],
        position: [0.0, 0.0, 0.0].into(),
        rotation: Rotation::new(2.0, 0.0, 0.0),
        scale: [1.0, 1.0, 1.0].into()
    };

    assert_eq!(mesh, expected);
}
