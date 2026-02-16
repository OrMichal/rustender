use crate::prelude::*;

#[allow(dead_code)]
pub struct Transferer<'a>(pub &'a mut AsciiBuffer, pub &'a Vec<Mesh>);

impl<'a> Transferer<'a> {
    pub fn start_transfering(&mut self, camera: &Camera, light_dir: Vec3) {
        self.1.iter()
            .for_each(|m| {
                m.triangles.iter()
                    .for_each(|t| { 
                        let normal = t.normal();
                        let intensity = normal.dot(light_dir.normalize()).max(0.0);
                        let ascii = ascii_from_intensity(intensity);

                        let rasterized_pos = t.rasterize();

                        rasterized_pos.iter()
                            .map(|p| p.get_projected_2d(camera.get_focal_length()))
                            .for_each(|v| {
                                let x = v.x as isize;
                                let y = v.y as isize;

                                if x >= 0 && y >= 0 && x < self.0.chunk_width as isize && y < (self.0.buffer.len() as isize / self.0.chunk_width as isize) {
                                    self.0.update_at(x as usize, y as usize, ascii);
                                }

                            })
                    })
            });
    }
}
