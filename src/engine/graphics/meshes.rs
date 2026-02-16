use crate::prelude::*;
#[allow(dead_code)]
pub struct MESHES;

impl MESHES {
    pub fn cube(size: f64) -> Mesh {
        let h = size / 2.0;

        let v = [
            Vec3::new(-h, -h, -h),
            Vec3::new( h, -h, -h),
            Vec3::new( h,  h, -h),
            Vec3::new(-h,  h, -h),
            Vec3::new(-h, -h,  h),
            Vec3::new( h, -h,  h),
            Vec3::new( h,  h,  h),
            Vec3::new(-h,  h,  h),
        ];

        let triangles = vec![
            Triangle::from([v[0], v[1], v[2]]),
            Triangle::from([v[0], v[2], v[3]]),

            Triangle::from([ v[5], v[4], v[7] ]),
            Triangle::from([ v[5], v[7], v[6] ]),

            Triangle::from([ v[4], v[0], v[3] ]),
            Triangle::from([ v[4], v[3], v[7] ]),

            Triangle::from([ v[1], v[5], v[6] ]),
            Triangle::from([ v[1], v[6], v[2] ]),

            Triangle::from([ v[4], v[5], v[1] ]),
            Triangle::from([ v[4], v[1], v[0] ]),

            Triangle::from([ v[3], v[2], v[6] ]),
            Triangle::from([ v[3], v[6], v[7] ]),
        ];

        Mesh {
            triangles,
            position: Vec3::new(0.0, 0.0, 10.0),
            rotation: Rotation::new(0.0, 0.0, 0.0),
        }
    }
}
