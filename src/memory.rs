pub struct Memory {
    memory: Box<[u8]>
}

impl Memory {
    pub fn new(size: usize) -> Self {
        let v = vec![0_u8; size];
        Self{memory: v.into_boxed_slice()}
    }

    pub fn getByte(&self, index: usize) -> u8 {
        return self.memory[index];
    }

    pub fn writeByte(&mut self, index: usize, value: u8) {
       self.memory[index] = value;
    }

    pub fn getBytes(&self, index: usize, bytes: usize) -> Box<[u8]> {
        let mut res  = vec![0_u8; bytes];

        for i in index..index+bytes {
            res[i] = self.getByte(i);
        }

        return res.into_boxed_slice();
    }
    
    pub fn writeBytes(&mut self, index: usize, value: &[u8]) {
        let bytes = value.len();

        for i in 0..bytes {
            self.writeByte(index + i, value[i]);
        }
    }

}
