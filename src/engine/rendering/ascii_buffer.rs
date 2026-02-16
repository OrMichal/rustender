use std::ops::{Index, IndexMut};
use crossterm::cursor::{MoveTo};
use crossterm::style::Print;
use std::io::{Write, stdout};

use crossterm::queue;

#[allow(dead_code)]
pub const ASCII_BRIGHTNESS: &[u8] = b" .:-=+*#%@";


pub fn ascii_from_intensity(intensity: f64) -> char {
    let idx = (intensity * (ASCII_BRIGHTNESS.len() as f64 - 1.0)).round() as usize;
    ASCII_BRIGHTNESS[idx] as char
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct AsciiBuffer {
    pub buffer: Vec<char>,
    pub chunk_width: u32
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

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn chunk(&self, start: usize, end: usize) -> Self {
        Self::new(self.chunk_width, self.buffer[start..end].to_vec().clone())
    }

    pub fn print(&self) {
        for i in 0..self.buffer.len() {
            let x = (i % self.chunk_width as usize) as u16;
            let y = (i / self.chunk_width as usize) as u16;

            queue!(stdout(), MoveTo(x, y), Print(self.buffer[i])).expect("Failed to print to console");
        }

        stdout().flush().expect("Failed to refresh buffer");
    }

    pub fn update_at(&mut self, i: usize, j: usize, value: char) {
        self.buffer[i + (j * self.chunk_width as usize)] = value;
    }
}

impl Index<(usize, usize)> for AsciiBuffer {
    type Output = char;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.buffer[index.0 + (index.1 * self.chunk_width as usize)]
    }
}

impl IndexMut<(usize, usize)> for AsciiBuffer {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.buffer[index.0 + (index.1 * self.chunk_width as usize)]
    }
}
