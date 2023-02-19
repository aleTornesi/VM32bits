use crate::memory_mapper::MemoryMappable;

fn move_to(x: u32, y: u32) {
    print!("\x1b[{};{}H", y, x);
}

pub struct ScreenDevice {}

impl MemoryMappable for ScreenDevice {
    fn get_byte(&self, _: u32) -> [u8; 1] {
        panic!("You can only write half words here")
    }

    fn get_half_word(&self, _: u32) -> [u8; 2] {
        panic!("You can only write half words here")
    }

    fn get_word(&self, _: u32) -> [u8; 4] {
        panic!("You can only write half words here")
    }

    fn write_byte(&mut self, address: u32, value: [u8; 1]) {
        self.write_half_word(address, (value[0] as u16).to_be_bytes());
    }

    fn write_half_word(&mut self, address: u32, value: [u8; 2]) {
        let value = u16::from_be_bytes(value);
        let character_value = value & 0x00ff;

        let x = (address % 16) + 1;
        let y = (address / 16) + 1;

        move_to(x , y);
        let character = char::from_u32(character_value as u32).unwrap();
        print!("{}", character)
    }

    fn write_word(&mut self, address: u32, value: [u8; 4]) {
        self.write_half_word(address, (u32::from_be_bytes(value) as u16).to_be_bytes())
    }
}