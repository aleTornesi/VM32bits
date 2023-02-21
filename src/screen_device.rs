use num_derive::FromPrimitive;

use crate::memory_mapper::MemoryMappable;

fn move_to(x: u32, y: u32) {
    print!("\x1b[{};{}H", y, x);
}

fn erase_screen() {
    print!("\x1b[2J")
}

fn set_bold() {
    print!("\x1b[1m");
}

fn set_regular() {
    print!("\x1b[0m");
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
        self.write_word(address, (u16::from_be_bytes(value) as u32).to_be_bytes())
    }

    fn write_word(&mut self, address: u32, value: [u8; 4]) {
        let value = u32::from_be_bytes(value);
        let character_value = value & 0x00ff;
        let command = (value & 0xff00) >> 8;
        println!("{}", command);
        let command: Command = num::FromPrimitive::from_u32(command).unwrap();


        match command {
            Command::ERASE_SCREEN => erase_screen(),
            Command::SET_BOLD => set_bold(),
            Command::SET_REGULAR => set_regular(),
            Command::NO_OP => {},
        }

        let x = (address % 32) + 1;
        let y = (address / 32) + 1;

        move_to(x , y);
        let character = char::from_u32(character_value).unwrap();
        print!("{}", character)
    }
}

#[derive(FromPrimitive)]
pub enum Command {
    ERASE_SCREEN = 0xff,
    SET_BOLD = 0x01,
    SET_REGULAR = 0x02,
    NO_OP = 0x00
}
