use crate::Vec3;

#[allow(dead_code)]
pub struct GDI;

impl GDI {
    pub fn line(start: Vec3, end: Vec3) -> Vec<Vec3> {
        let mut points = Vec::new();

        let dx = (end.x - start.x).abs() as i64;
        let dy = -(end.y - start.y).abs() as i64;
        let sx = if start.x < end.x { 1 } else { -1 };
        let sy = if start.y < end.y { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = start.x as i64;
        let mut y = start.y as i64;
        let z = start.z;

        loop {
            points.push(Vec3::new(x as f64, y as f64, z));
            if x == end.x as i64 && y == end.y as i64 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }

        points
    }
}
