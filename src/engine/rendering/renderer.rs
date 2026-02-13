use std::thread;

use crate::engine::{graphics::Size, rendering::{AsciiBuffer, RenderQuality}};

#[allow(dead_code)]
pub struct Renderer {
    front_buffer: AsciiBuffer,
    back_buffer: AsciiBuffer,    
    buffer_size: Size,
    fps: i16,
    quality: RenderQuality,
    on_failed: Box<dyn Fn(&'static str) -> ()>
}

impl Renderer {
    pub fn new() -> RendererBuilder {
        RendererBuilder { buffer_size: None, fps: 60, quality: None }
    }

    pub fn on_error<T>(&mut self, f: T)
        where T: Fn(&'static str) + 'static {
        self.on_failed = Box::new(f);
    }

    pub fn render(&self) {
        let chunk_size = self.front_buffer.len() / self.quality.clone() as usize;

        thread::scope(|s| {
            for i in 0..(self.quality.clone() as usize) {
                let start = i * chunk_size;
                let end = if i == 3 {
                    self.front_buffer.len()
                } else {
                    (i + 1) * chunk_size
                };

                let slice = self.front_buffer.chunk(start, end);

                s.spawn(move || {
                    slice.print();
                });
            }
        });
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
                None => 100 * 50 as usize
            }]), 
            back_buffer: AsciiBuffer::new(self.buffer_size.clone().unwrap().width as u32, vec![' '; match &self.buffer_size {
                Some(size) => (size.height * size.width) as usize,
                None => 100 * 50 as usize
            }]), 
            fps: self.fps, 
            quality: match &self.quality {
                Some(q) => q.clone(),
                None => RenderQuality::Low
            }, 
            on_failed: Box::new(|s| {
                println!("{}", s);
            }) 
        }
    }

    pub fn set_fps(mut self, fps: i16) -> Self {
        self.fps = fps;

        self
    }

    pub fn set_quality(mut self, quality: RenderQuality) -> Self {
        self.quality = Some(quality);

        self
    }

    pub fn set_size(mut self, size: Size) -> Self {
        self.buffer_size = Some(size);

        self
    }

    pub fn set_width(mut self, width: f32) -> Self {
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


    pub fn set_height(mut self, height: f32) -> Self {
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
}
