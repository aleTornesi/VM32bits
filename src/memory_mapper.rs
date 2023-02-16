use crate::memory::Memory;

pub struct MemoryMapper {
    regions: Vec<Region>,
}

impl MemoryMapper {
    pub fn new() -> MemoryMapper {
        MemoryMapper { regions: vec![] }
    }

    fn map(&mut self, device: Memory, start: u32, end: u32, remap: bool) -> &Region {
        self.regions.insert(0, Region{device, start, end, remap});
        return self.regions.get(0).unwrap();
    }

    fn unmap(&mut self, region: &Region) {
        self.regions.retain(|r| region != r);
    }
    
    fn find_region(&self, address: u32) -> &Region {
        return self.regions.iter().find(|r| r.start <= address && address <= r.end).unwrap();
    }

    fn find_mut_region(&mut self, address: u32) -> &mut Region {
        return self.regions.iter_mut().find(|r| r.start <= address && address <= r.end).unwrap();
    }

    fn remap_address(region: &Region, address: u32) -> u32 {
        if region.remap {
            return address - region.start;
        }
        return address;
    }

    fn get_byte(&self, address: u32) -> [u8; 1] {
        let region = self.find_region(address);
        let final_address = MemoryMapper::remap_address(region, address);
        return region.device.get_byte(final_address as usize);
    }

    fn get_half_word(&self, address: u32) -> [u8; 2] {
        let region = self.find_region(address);
        let final_address = MemoryMapper::remap_address(region, address);
        return region.device.get_half_word(final_address as usize);
    }

    fn get_word(&self, address: u32) -> [u8; 4] {
        let region = self.find_region(address);
        let final_address = MemoryMapper::remap_address(region, address);
        return region.device.get_word(final_address as usize);
    }

    fn set_byte(&mut self, address: u32, value:u8) {
        let region = self.find_mut_region(address);
        let final_address = MemoryMapper::remap_address(region, address);
        region.device.write_byte(final_address as usize, value);
    }
    
    fn set_half_word(&mut self, address: u32, value:[u8; 2]) {
        let region = self.find_mut_region(address);
        let final_address = MemoryMapper::remap_address(region, address);
        region.device.write_half_word(final_address as usize, value);
    }

    fn set_word(&mut self, address: u32, value: [u8; 4]) {
        let region = self.find_mut_region(address);
        let final_address = MemoryMapper::remap_address(region, address);
        region.device.write_word(final_address as usize, value);
    }
}



#[derive(PartialEq)]
struct Region {
    device: Memory,
    start: u32,
    end: u32,
    remap: bool
}