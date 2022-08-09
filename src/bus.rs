#[derive(Debug)]
pub(crate) struct Bus {
    memory: Vec<u8>,
}

impl Bus {
    pub fn new(memory: Vec<u8>) -> Self {
        Self {
            memory
        }
    }

    pub fn get_code8(&self, eip: usize, index: usize) -> u8 {
        self.memory[eip + index]
    }

    pub fn get_sign_code8(&self, eip: usize, index: usize) -> i8 {
        self.memory[eip + index] as i8
    }

    pub fn get_code32(&self, eip: usize, index: usize) -> u32 {
        (self.memory[eip + index + 3] as u32) << 24
            | (self.memory[eip + index + 2] as u32) << 16
            | (self.memory[eip + index + 1] as u32) << 8
            | (self.memory[eip + index + 0] as u32)
    }

    pub fn get_sign_code32(&self, eip: usize, index: usize) -> i32 {
        (self.memory[eip + index + 3] as i32) << 24
            | (self.memory[eip + index + 2] as i32) << 16
            | (self.memory[eip + index + 1] as i32) << 8
            | (self.memory[eip + index + 0] as i32)
    }

    pub fn set_memory8(&mut self, address: u32, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn set_memory32(&mut self, addres: u32, value: u32) {
        for i in 0..4 {
            self.set_memory8(addres + i, (value >> (i * 8) & 0xFF) as u8);
        }
    }
}
