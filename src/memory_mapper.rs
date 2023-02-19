use std::ptr;

pub struct MemoryMapper {
    regions: Vec<Region>,
}

impl MemoryMapper {
    pub fn new() -> Self {
        MemoryMapper { regions: vec![]  }
    }

    pub fn map(&mut self, device: Box<dyn MemoryMappable>, start: u32, end: u32, remap: bool) -> &Region {
        self.regions.insert(0, Region{device: device, start, end, remap});
        return self.regions.get(0).unwrap();
    }

    #[allow(dead_code)]
    pub fn unmap<T: MemoryMappable>(&mut self, region: &Region) {
        self.regions.retain(|r| !ptr::eq(region, r));
    }
    
    pub fn find_region(&self, address: u32) -> &Region {
        return self.regions.iter().find(|r| r.start <= address && address <= r.end).unwrap();
    }

    pub fn find_mut_region(&mut self, address: u32) -> &mut Region {
        return self.regions.iter_mut().find(|r| r.start <= address && address <= r.end).unwrap();
    }

    fn remap_address(region: &Region, address: u32) -> u32 {
        if region.remap {
            return address - region.start;
        }
        return address;
    }

    pub fn get_byte(&self, address: u32) -> [u8; 1] {
        let region = self.find_region(address);
        let final_address = MemoryMapper::remap_address(region, address);
        return region.device.get_byte(final_address);
    }

    pub fn get_half_word(&self, address: u32) -> [u8; 2] {
        let region = self.find_region(address);
        let final_address = MemoryMapper::remap_address(region, address);
        return region.device.get_half_word(final_address);
    }

    pub fn get_word(&self, address: u32) -> [u8; 4] {
        let region = self.find_region(address);
        let final_address = MemoryMapper::remap_address(region, address);
        return region.device.get_word(final_address);
    }

    pub fn write_byte(&mut self, address: u32, value:[u8; 1]) {
        let region = self.find_mut_region(address);
        let final_address = MemoryMapper::remap_address(region, address);
        region.device.write_byte(final_address, value);
    }
    
    pub fn write_half_word(&mut self, address: u32, value:[u8; 2]) {
        let region = self.find_mut_region(address);
        let final_address = MemoryMapper::remap_address(region, address);
        region.device.write_half_word(final_address, value);
    }

    pub fn write_word(&mut self, address: u32, value: [u8; 4]) {
        let region = self.find_mut_region(address);
        let final_address = MemoryMapper::remap_address(region, address);
        region.device.write_word(final_address, value);
    }
}


pub trait MemoryMappable {
    fn get_byte(&self, address: u32) -> [u8; 1];
    fn get_half_word(&self, address: u32) -> [u8; 2];
    fn get_word(&self, address: u32) -> [u8; 4];
    fn write_byte(&mut self, address: u32, value: [u8; 1]);
    fn write_half_word(&mut self, address: u32, value: [u8; 2]);
    fn write_word(&mut self, address: u32, value: [u8; 4]);
}

//#[derive(PartialEq)]
pub struct Region {
    device: Box<dyn MemoryMappable>,
    start: u32,
    end: u32,
    remap: bool
}