#[allow(dead_code)]
pub struct AsciiBuffer {
    buffer: Vec<char>,
    chunk_width: u32
}

impl AsciiBuffer {
    pub fn new(chunk_width: u32, buff: Vec<char>) -> Self {
        Self {
            buffer: buff,
            chunk_width
        }
    }

    pub fn set(& mut self, buff: Vec<char>) {
        self.buffer = buff;
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn chunk(&self, start: usize, end: usize) -> Self {
        Self::new(self.chunk_width, self.buffer[start..end].to_vec().clone())
    }

    pub fn print(&self) {
            for i in 0..self.buffer.len() {
                let x = i % self.chunk_width as usize;
                let y = i / self.chunk_width as usize;

                crossterm::cursor::MoveTo(x as u16, y as u16);
                crossterm::style::Print(self.buffer[i]);
            }
    }
}
