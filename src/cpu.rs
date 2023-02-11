use crate::memory::Memory;
pub struct CPU {
    registers: [u32; 32],
    fRegisters: [f32; 32],
    memory: Memory,
    pc: usize
    //index: u32
}

impl CPU {
    const REGISTER_MASK: u32 = 0b00000011111;
    const FUNCTION_MASK: u32 = 0x0000003f;
    const IMMEDIATE_MASK: u32 = 0x0000ffff;

    pub fn new(memory: Memory) -> CPU {
        CPU{ registers: [0; 32], fRegisters: [0_f32; 32], memory, pc: 0 /*index: 0*/ }
    }

    fn fetch(&mut self) -> u32 {
        let mut instructionBytes:[u8; 4] = [0; 4];
        for (i, v) in self.memory.getBytes(self.pc, 4).into_iter().enumerate() {
            instructionBytes[i] = *v;
        }
        let res = u32::from_ne_bytes(instructionBytes);
        return res;
    }

    pub fn execute(&mut self, instruction: u32) {
        let op_code = instruction >> 26;
        println!("{:b}", op_code);
        match op_code {
            Instruction::R => {
                let rs:u8 = ((instruction >> 21) & CPU::REGISTER_MASK) as u8;
                let rt:u8 = ((instruction >> 16) & CPU::REGISTER_MASK) as u8;
                let rd:u8 = ((instruction >> 11) & CPU::REGISTER_MASK) as u8;
                let shift_amount: u8 = ((instruction >> 6) & CPU::REGISTER_MASK) as u8;
                let function: u8 = (instruction & CPU::FUNCTION_MASK) as u8;
                self.alu_operation(rs, rt, rd, shift_amount, function);
            },
            Instruction::ADD_IMMEDIATE => {
                let parameters = CPU::get_r_immediate_instructions_values(instruction);
                let result: i32 = i32::from_ne_bytes(self.registers[parameters.0 as usize].to_ne_bytes()) + i32::from_ne_bytes(parameters.2.to_ne_bytes());
                self.registers[parameters.1 as usize] =  u32::from_ne_bytes(result.to_ne_bytes())
            },
            Instruction::ADD_IMMEDIATE_UNSIGNED => {
                let parameters = CPU::get_r_immediate_instructions_values(instruction);
                let result: u32 = u32::from_be_bytes((self.registers[parameters.0 as usize] + parameters.2 as u32).to_be_bytes());
                self.registers[parameters.1 as usize] = result;
            },
            Instruction::SUB_IMMEDIATE => {
                let parameters = CPU::get_r_immediate_instructions_values(instruction);
                let result: i32 = i32::from_ne_bytes(self.registers[parameters.0 as usize].to_ne_bytes()) - i32::from_ne_bytes(parameters.2.to_ne_bytes());
                self.registers[parameters.1 as usize] =  u32::from_ne_bytes(result.to_ne_bytes())
            },
            Instruction::SUB_IMMEDIATE_UNSIGNED => {
                let parameters = CPU::get_r_immediate_instructions_values(instruction);
                let result: u32 = u32::from_be_bytes((self.registers[parameters.0 as usize] - parameters.2 as u32).to_be_bytes());
                self.registers[parameters.1 as usize] = result;
            },
            Instruction::MUL_IMMEDIATE => {
                let parameters = CPU::get_r_immediate_instructions_values(instruction);
                let result: i32 = i32::from_ne_bytes(self.registers[parameters.0 as usize].to_ne_bytes()) * i32::from_ne_bytes(parameters.2.to_ne_bytes());
                self.registers[parameters.1 as usize] =  u32::from_ne_bytes(result.to_ne_bytes())
            },
            Instruction::MUL_IMMEDIATE_UNSIGNED => {
                let parameters = CPU::get_r_immediate_instructions_values(instruction);
                let result: u32 = u32::from_be_bytes((self.registers[parameters.0 as usize] * parameters.2 as u32).to_be_bytes());
                self.registers[parameters.1 as usize] = result;
            },
            Instruction::DIV_IMMEDIATE => {
                let parameters = CPU::get_r_immediate_instructions_values(instruction);
                let result: i32 = i32::from_ne_bytes(self.registers[parameters.0 as usize].to_ne_bytes()) / i32::from_ne_bytes(parameters.2.to_ne_bytes());
                self.registers[parameters.1 as usize] =  u32::from_ne_bytes(result.to_ne_bytes())
            },
            Instruction::DIV_IMMEDIATE_UNSIGNED => {
                let parameters = CPU::get_r_immediate_instructions_values(instruction);
                let result: u32 = u32::from_be_bytes((self.registers[parameters.0 as usize] / parameters.2 as u32).to_be_bytes());
                self.registers[parameters.1 as usize] = result;
            },


            _ => {}
        }
    }

    fn get_r_immediate_instructions_values(instruction: u32) -> (u8, u8, u32) {
        let rs = ((instruction >> 21) & CPU::REGISTER_MASK) as u8;
        let rd = ((instruction >> 16) & CPU::REGISTER_MASK) as u8;
        let immediate = (instruction & CPU::IMMEDIATE_MASK) as u32;
        return (rs, rd, immediate);
    }

    pub fn get_register_value(&self, v: u8) -> u32 {
        if v >= 32 {
            panic!("This register does not exist");
        }
        return self.registers[v as usize];
    }

    fn alu_operation(&self, rs:u8, rt:u8, rd:u8, shift_amount: u8, function: u8) {

    }

}



struct Instruction{}
impl Instruction {
    const R: u32 = 0o00;
    const LW: u32 = 0o01;
    const SW: u32 = 0o02;
    const ADD_IMMEDIATE: u32 = 0o03;
    const ADD_IMMEDIATE_UNSIGNED: u32 = 0o04;
    const SUB_IMMEDIATE: u32 = 0o05;
    const SUB_IMMEDIATE_UNSIGNED: u32 = 0o06;
    const MUL_IMMEDIATE: u32 = 0o07;
    const MUL_IMMEDIATE_UNSIGNED: u32 = 0o10;
    const DIV_IMMEDIATE: u32 = 0o11;
    const DIV_IMMEDIATE_UNSIGNED: u32 = 0o12;
    const F_DIV_IMMEDIATE: u32 = 0o13;
    const F_MUL_IMMEDIATE: u32 = 0o07;
    const AND_IMMEDIATE: u32 = 0o15;
    const OR_IMMEDIATE: u32 = 0o16;
    const XOR_IMMEDIATE: u32 = 0o16;
    const LEFT_SHIFT_IMMEDIATE: u32 = 0o17;
    const RIGHT_SHIFT_IMMEDIATE: u32 = 0o20;
    const LOAD_IMMEDIATE: u32 = 0o21;
    const LOAD_UPPER_IMMEDIATE: u32 = 0o22;
    const MOVE_FORM_HI: u32 = 0o23;
    const MOVE_FORM_LO: u32 = 0o24;
    const BRANCH_ON_EQUAL: u32 = 0o25;
    const BRANCH_ON_NOT_EQUAL: u32 = 0o26;
    const BRANCH_ON_GRATER_THAN: u32 = 0o26;
    const BRANCH_ON_GRATER_THAN_OR_EQUAL: u32 = 0o27;
    const BRANCH_ON_LESS_THAN: u32 = 0o30;
    const BRANCH_ON_LESS_THAN_OR_EQUAL: u32 = 0o31;
    const SET_LESS_THAN: u32 = 0o32;
    const SET_LESS_THAN_IMMEDIATE: u32 = 0o33;
    const JUMP: u32 = 0o34;
    const JUMP_REGISTER: u32 = 0o35;
    const JUMP_AND_LINK: u32 = 0o36;
    const F_ADD_IMMEDIATE: u32 = 0o37;
    const F_SUB_IMMEDIATE: u32 = 0o40;
}
