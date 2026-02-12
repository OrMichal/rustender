use crate::engine::{graphics::Size, rendering::RenderQuality};

#[allow(dead_code)]
pub struct Renderer {
    buffer: Vec<char>,
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
            buffer: vec![' '; match &self.buffer_size {
                Some(size) => (size.height * size.width) as usize,
                None => 100 * 50 as usize
            }], 
            buffer_size: match &self.buffer_size {
                Some(size) => size.clone(),
                None => Size::new(100.0, 50.0)
            }, 
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
