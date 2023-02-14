pub struct Memory {
    memory: Box<[u8]>
}

impl Memory {
    pub fn new(size: usize) -> Self {
        let v = vec![0_u8; size];
        Self{memory: v.into_boxed_slice()}
    }

    pub fn get_byte(&self, index: usize) -> [u8; 1] {
        return [self.memory[index]];
    }

    pub fn write_byte(&mut self, index: usize, value: u8) {
       self.memory[index] = value;
    }

    pub fn get_half_word(&self, index: usize) -> [u8; 2] {
        return [self.memory[index], self.memory[index+1]];
    }

    pub fn write_half_word(&mut self, index: usize, content: [u8; 2]) {
        for (i, v) in content.iter().enumerate() {
            self.memory[index + i] = *v;
        }
    }

    pub fn write_word(&mut self, index: usize, content: [u8; 4]) {
        for (i, v) in content.iter().enumerate() {
            self.memory[index + i] = *v;
        }
    }

    pub fn get_word(&self, index: usize) -> [u8; 4] {
        return [self.memory[index], self.memory[index+1], self.memory[index+2], self.memory[index+3]]
    }

}
