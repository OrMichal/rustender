use crate::prelude::*;

#[allow(dead_code)]
pub struct Transferer<'a>(pub &'a mut AsciiBuffer, pub &'a Vec<Mesh>);

impl<'a> Transferer<'a> {
    pub fn start_transfering(&mut self, camera: &Camera, light_dir: Vec3) {
        self.1.iter().for_each(|m| {
            m.triangles.iter().for_each(|t| {
                let normal = t.normal();
                let intensity = normal.dot(light_dir.normalize()).max(0.0);
                let ascii = ascii_from_intensity(intensity);

                // Project vertices to 2D FIRST
                let projected_vertices: [Vec2; 3] = [
                    t.vertices[0].get_projected_2d(camera.get_focal_length()),
                    t.vertices[1].get_projected_2d(camera.get_focal_length()),
                    t.vertices[2].get_projected_2d(camera.get_focal_length()),
                ];

                // Then rasterize in 2D screen space
                let rasterized_pos = Triangle::rasterize_2d_triangle(projected_vertices);

                rasterized_pos.iter().for_each(|v| {
                    // Center on screen
                    let x = (v.x + (camera.width as f64 / 2.0)) as isize;
                    let y = (v.y + (camera.height as f64 / 2.0)) as isize;

                    if x >= 0
                        && y >= 0
                        && x < self.0.chunk_width as isize
                        && y < (self.0.buffer.len() as isize / self.0.chunk_width as isize)
                    {
                        self.0.update_at(x as usize, y as usize, ascii);
                    }
                })
            })
        });
    }
}
