use crate::memory_mapper::MemoryMappable;

#[derive(PartialEq)]
pub struct Memory {
    memory: Box<[u8]>
}

impl Memory {
    pub fn new(size: usize) -> Self {
        let v = vec![0_u8; size];
        Self{memory: v.into_boxed_slice()}
    }

    #[allow(dead_code)]
    pub fn get_size(&self) -> usize {
        return self.memory.len()
    }
}


impl MemoryMappable for Memory {
    fn get_byte(&self, index: u32) -> [u8; 1] {
        return [self.memory[index as usize]];
    }
    
    fn write_byte(&mut self, index: u32, value: [u8; 1]) {
       self.memory[index as usize] = value[0];
    }
    
    fn get_half_word(&self, index: u32) -> [u8; 2] {
        return [self.memory[index as usize], self.memory[index as usize+1]];
    }
    
    fn write_half_word(&mut self, index: u32, content: [u8; 2]) {
        for (i, v) in content.iter().enumerate() {
            self.memory[index as usize + i] = *v;
        }
    }
    
    fn write_word(&mut self, index: u32, content: [u8; 4]) {
        for (i, v) in content.iter().enumerate() {
            self.memory[index as usize + i] = *v;
        }
    }
    
    fn get_word(&self, index: u32) -> [u8; 4] {
        return [self.memory[index as usize], self.memory[index as usize+1], self.memory[index as usize+2], self.memory[index as usize+3]]
    }
    
    
}