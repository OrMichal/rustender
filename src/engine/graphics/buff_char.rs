use crate::prelude::*;

#[allow(dead_code)]
pub struct BuffChar {
    char: char,
    location: Vec2
}

impl BuffChar {
    pub fn new(location: Vec2, char: char) -> Self {
        Self { char, location }
    }
}
