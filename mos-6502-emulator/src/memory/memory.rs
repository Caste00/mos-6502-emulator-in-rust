const RAM_SIZE: usize = 1024 * 64;

#[derive(Debug)]
pub struct Memory {
    pub data: [u8; RAM_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: [0; RAM_SIZE],
        }
    }
}