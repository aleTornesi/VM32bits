mod cpu;
mod memory;
mod memory_mapper;
mod screen_device;
pub mod fpu;

use memory::Memory;
use cpu::CPU;
use cpu::Instruction;
use screen_device::ScreenDevice;
use memory_mapper::MemoryMapper;
use vm32bits::screen_device::Command;

fn main() {
    let mem = Memory::new(256 * 256);
    let sd = ScreenDevice{};

    let mut memory_mapper = MemoryMapper::new();
    memory_mapper.map(Box::new(mem), 0, 0xffff, false);
    memory_mapper.map(Box::new(sd), 0x9000, 0x90ff, true);

    let mut index = 0;

    //print_string(&mut memory_mapper, "Hello World!".to_owned(), &mut index);
    for i in 0..0xff {
        print_char(&mut memory_mapper, &mut index, '*', i as u8, Some(Command::ERASE_SCREEN));
    }

    let instruction = 0b1010_001100_u32;
    memory_mapper.write_word(index, instruction.to_be_bytes());

    let mut cpu: CPU = CPU::new(&mut memory_mapper);
    cpu.run();


    fn print_char(memory_mapper: &mut MemoryMapper, address: &mut u32, char: char, index: u8, command: Option<Command>) {
        let command = command.unwrap_or(Command::NO_OP);
        let instruction = form_i_instruction(Instruction::ADDIU as u32, 0, 1, char as u32 + ((command as u32) << 8));
    
        memory_mapper.write_word(*address, instruction.to_be_bytes());
        *address += 4;

        let instruction = form_i_instruction(Instruction::SB as u32, 0, 1, 0x9000 + index as u32);
        memory_mapper.write_word(*address, instruction.to_be_bytes());

        *address += 4;
    }

    fn print_string(memory_mapper: &mut MemoryMapper, s: String, address: &mut u32) {
        for (i, c) in s.chars().into_iter().enumerate() {
            print_char(memory_mapper, address, c, (i) as u8, Some(Command::ERASE_SCREEN));
        }
    } 
}


fn form_i_instruction(op_code: u32, rs: u32, rd: u32, immediate: u32) -> u32 {
    return (op_code << 26) + (rs << 21) + (rd << 16) + immediate;
}
