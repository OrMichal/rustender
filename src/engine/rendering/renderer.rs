use std::{io::{Write, stdout}};
use crossterm::queue;
use crate::{Camera, Mesh, Transferer, Vec3, engine::{graphics::Size, rendering::{AsciiBuffer, RenderQuality}}};

#[allow(dead_code)]
pub struct Renderer {
    front_buffer: AsciiBuffer,
    back_buffer: AsciiBuffer,    
    buffer_size: Size,
    fps: i16,
    quality: RenderQuality,
    on_failed: Box<dyn Fn(&'static str)>,
    light_direction: Vec3,
    meshes: Vec<Mesh>
}

#[allow(dead_code)]
impl Renderer {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> RendererBuilder {
        RendererBuilder { buffer_size: None, fps: 60, quality: None, light_direction: None }
    }

    pub fn on_error<T: Fn(&'static str) + 'static>(&mut self, f: T) {
        self.on_failed = Box::new(f);
    }

    fn render(&self) {
        self.front_buffer.print();
    }

    pub fn start(&mut self, camera: &Camera) {
        let mut stdout = stdout();
        queue!(stdout, crossterm::cursor::Hide).unwrap();

        let frame_time = std::time::Duration::from_millis(1000 / self.fps as u64);

        loop {
            let start = std::time::Instant::now();
            {
                let mut transferer  = Transferer(&mut self.back_buffer, &self.meshes);

                transferer.start_transfering(camera, self.light_direction);

                self.sync_buffers();
            }

            self.render();
            stdout.flush().unwrap();

            let elapsed = start.elapsed();
            if elapsed < frame_time {
                std::thread::sleep(frame_time - elapsed);
            }
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }

    fn sync_buffers(&mut self) {
        std::mem::swap(&mut self.front_buffer, &mut self.back_buffer);
    }

    fn update_buffer(&mut self) {
        std::mem::swap(&mut self.front_buffer, &mut self.back_buffer);
    }

    fn calculate_next_scene(&mut self) {

    }
}

#[allow(dead_code)]
pub struct RendererBuilder {
    buffer_size: Option<Size>,
    fps: i16,
    quality: Option<RenderQuality>,
    light_direction: Option<Vec3>
}

impl RendererBuilder {
    pub fn build(self) -> Renderer {
        Renderer { 
            buffer_size: match &self.buffer_size {
                Some(size) => size.clone(),
                None => Size::new(100.0, 50.0)
            }, 
            front_buffer: AsciiBuffer::new(self.buffer_size.clone().unwrap().width as u32, vec![' '; match &self.buffer_size {
                Some(size) => (size.height * size.width) as usize,
                None => 100 * 50
            }]), 
            back_buffer: AsciiBuffer::new(self.buffer_size.clone().unwrap().width as u32, vec![' '; match &self.buffer_size {
                Some(size) => (size.height * size.width) as usize,
                None => 100 * 50
            }]), 
            fps: self.fps, 
            quality: match &self.quality {
                Some(q) => q.clone(),
                None => RenderQuality::Low
            }, 
            on_failed: Box::new(|s| {
                println!("{s}");
            }),
            light_direction: match &self.light_direction {
                Some(l) => *l,
                None => Vec3::new(0.0, 0.0, -1.0).normalize()
            },
            meshes: vec![]
        }
    }

    pub fn fps(mut self, fps: i16) -> Self {
        self.fps = fps;

        self
    }

    pub fn quality(mut self, quality: RenderQuality) -> Self {
        self.quality = Some(quality);

        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.buffer_size = Some(size);

        self
    }

    pub fn width(mut self, width: f32) -> Self {
        match & mut self.buffer_size {
            Some(size) => {
                size.width = width;
            },
            None => {
                self.buffer_size = Some(Size::new(width, 0.0));
            }
        }

        self
    }


    pub fn height(mut self, height: f32) -> Self {
        match & mut self.buffer_size {
            Some(size) => {
                size.height = height;
            },
            None => {
                self.buffer_size = Some(Size::new(0.0, height));
            }
        }

        self
    }

    pub fn light_direction(mut self, direction: Vec3) -> Self {
        self.light_direction = Some(direction);

        self
    }
}
