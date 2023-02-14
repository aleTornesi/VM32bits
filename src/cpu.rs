extern crate num;

use num_derive::FromPrimitive;

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
        for (i, v) in self.memory.get_word(self.pc).into_iter().enumerate() {
            instructionBytes[i] = v;
        }
        let res = u32::from_be_bytes(instructionBytes);
        return res;
    }

    pub fn get_register_value(&self, i: usize) -> u32 {
        return self.registers[i];
    }

    pub fn execute(&mut self, instruction: u32) {
        let op_code: u8 = (instruction >> 26) as u8;
        let op_code: Instruction = num::FromPrimitive::from_u8(op_code).unwrap();
        match op_code {
            Instruction::R => {
                let rs:u8 = ((instruction >> 21) & CPU::REGISTER_MASK) as u8;
                let rt:u8 = ((instruction >> 16) & CPU::REGISTER_MASK) as u8;
                let rd:u8 = ((instruction >> 11) & CPU::REGISTER_MASK) as u8;
                let shift_amount: u8 = ((instruction >> 6) & CPU::REGISTER_MASK) as u8;
                let function: u8 = (instruction & CPU::FUNCTION_MASK) as u8;
                self.alu_operation(rs, rt, rd, shift_amount, function);
            },
            Instruction::ADDI => {
                let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                let rs_value = i32::from_be_bytes(self.registers[rs as usize].to_be_bytes());
                let immediate_signed_interpretation = i32::from_be_bytes(immediate.to_be_bytes());
                let result = rs_value + immediate_signed_interpretation;
                self.registers[rt as usize] = u32::from_be_bytes(result.to_be_bytes());
            },
            Instruction::ADDIU => {
                let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                let rs_value = self.registers[rs as usize];
                let result = rs_value + immediate;
                self.registers[rt as usize] = result;
            },
            Instruction::LB => {
                let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize];
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let signed_content = i8::from_be_bytes(self.memory.get_byte((index as i32 + offset) as usize));
                self.registers[rt as usize] =  u32::from_be_bytes((signed_content as i32).to_be_bytes());
            },
            Instruction::LBU => {
                let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize] as i32;
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let content = u8::from_be_bytes(self.memory.get_byte((index + offset) as usize));
                self.registers[rt as usize] = content as u32;
            },
            Instruction::LHW => {
                let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize] as i32;
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let signed_content = i16::from_be_bytes(self.memory.get_half_word((index + offset) as usize));
                self.registers[rt as usize] =  u32::from_be_bytes((signed_content as i32).to_be_bytes());
            },
            Instruction::LHWU => {
                let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize] as i32;
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let content = u16::from_be_bytes(self.memory.get_half_word((index + offset) as usize));
                self.registers[rt as usize] = content as u32;
            },
            Instruction::LW => {
                let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize];
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let signed_content = i32::from_be_bytes(self.memory.get_word((index as i32 + offset) as usize));
                self.registers[rt as usize] =  u32::from_be_bytes((signed_content as i32).to_be_bytes());
            },
            Instruction::LUI => {
                let (_, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                self.registers[rt as usize] = immediate << 16;
            },
            Instruction::LWCz => {

            },
            Instruction::LWL => {

            },
            Instruction::LWR => {
                // let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                // let mut address = self.registers[rs as usize] + immediate;
                // let mut value = self.registers[rt as usize];
                // if address % 4 == 0 {
                //     value += u32::from_be_bytes(self.memory.get_word(address as usize));
                // } else {
                //     let mut i = address % 4;
                //     while address % 4 != 0 {
                //         value += (u8::from_be_bytes(self.memory.get_byte(address as usize)) as u32) * 2_u32.pow(8 * (i-1));
                //         address += 1;
                //         i -= 1;
                //     }
                // }

                // self.registers[rt as usize] = value;
            },
            Instruction::SB => {
                let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize];
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let signed_content = self.registers[rt as usize] as u8;
                self.memory.write_byte((index as i32 + offset) as usize, signed_content);
            },
            Instruction::SHW => {
                let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize];
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let signed_content = self.registers[rt as usize] as u16;
                self.memory.write_half_word((index as i32 + offset) as usize, signed_content.to_be_bytes());
            },
            Instruction::SW => {
                let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize];
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let signed_content = self.registers[rt as usize];
                self.memory.write_word((index as i32 + offset) as usize, signed_content.to_be_bytes());
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

    fn alu_operation(&mut self, rs:u8, rt:u8, rd:u8, shift_amount: u8, function: u8) {
        if rd == 0 {
            panic!("You cannot write on the zero register");
        }

        let function: Function = num::FromPrimitive::from_u8(function).unwrap();
        match function {
            Function::ADD => {
                let rs_value = i32::from_be_bytes(self.registers[rs as usize].to_be_bytes());
                let rt_value = i32::from_be_bytes(self.registers[rt as usize].to_be_bytes());
                let result: i32 = (rs_value + rt_value) << shift_amount;
                self.registers[rd as usize] =  u32::from_be_bytes(result.to_be_bytes())
            },
            Function::ADDU => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = (rs_value + rt_value) << shift_amount;
                self.registers[rd as usize] = result;
            },
            Function::SUB => {
                let rs_value = i32::from_be_bytes(self.registers[rs as usize].to_be_bytes());
                let rt_value = i32::from_be_bytes(self.registers[rt as usize].to_be_bytes());
                let result: i32 = (rs_value - rt_value) << shift_amount;
                self.registers[rd as usize] =  u32::from_be_bytes(result.to_be_bytes())
            },
            Function::SUBU => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = (rs_value - rt_value) << shift_amount;
                self.registers[rd as usize] = result;
            },
            Function::MULT => {
                let rs_value = i32::from_be_bytes(self.registers[rs as usize].to_be_bytes());
                let rt_value = i32::from_be_bytes(self.registers[rt as usize].to_be_bytes());
                let result: i32 = (rs_value * rt_value) << shift_amount;
                self.registers[rd as usize] =  u32::from_be_bytes(result.to_be_bytes())
            },
            Function::MULTU => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = (rs_value * rt_value) << shift_amount;
                self.registers[rd as usize] = result;
            },
            Function::DIV => {
                let rs_value = i32::from_be_bytes(self.registers[rs as usize].to_be_bytes());
                let rt_value = i32::from_be_bytes(self.registers[rt as usize].to_be_bytes());
                let result: i32 = (rs_value / rt_value) << shift_amount;
                self.registers[rd as usize] =  u32::from_be_bytes(result.to_be_bytes())
            },
            Function::DIVU => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = (rs_value / rt_value) << shift_amount;
                self.registers[rd as usize] = result;
            },
            Function::AND => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = (rs_value & rt_value) << shift_amount;
                self.registers[rd as usize] = result;
            },
            Function::OR => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = (rs_value | rt_value) << shift_amount;
                self.registers[rd as usize] = result;
            },
            Function::XOR => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = (rs_value ^ rt_value) << shift_amount;
                self.registers[rd as usize] = result;
            },
            Function::NOR => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = !(rs_value | rt_value) << shift_amount;
                self.registers[rd as usize] = result;
            },
            Function::SLL => {
                let rt_value = self.registers[rt as usize];
                let result = rt_value << shift_amount;
                self.registers[rd as usize] = result;
            },
            Function::SRL => {
                let rt_value = self.registers[rt as usize];
                let result = rt_value >> shift_amount;
                self.registers[rd as usize] = result;
            },
            Function::SLLV => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = rs_value << rt_value;
                self.registers[rd as usize] = result;
            },
            Function::SRLV => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = rs_value >> rt_value;
                self.registers[rd as usize] = result;
            },
            Function::SLT => {
                let rs_value = i32::from_be_bytes(self.registers[rs as usize].to_be_bytes());
                let rt_value = i32::from_be_bytes(self.registers[rt as usize].to_be_bytes());
                let result: i32 = ((rs_value < rt_value) as i32)  << shift_amount;
                self.registers[rd as usize] =  u32::from_be_bytes(result.to_be_bytes())
            },
            Function::SLTU => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = ((rs_value < rt_value) as u32)  << shift_amount;
                self.registers[rd as usize] =  result;
            },
            Function::SRA => {
                let rt_value = self.registers[rt as usize];
                let result = rt_value * (2_u32.pow(shift_amount as u32));
                self.registers[rd as usize] = result;
            }
            Function::SRAV => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = rs_value * (2_u32.pow(rt_value as u32));
                self.registers[rd as usize] = result;
            }

            
            
           _ => {}
        }
    }
    
}


#[derive(FromPrimitive)]
enum Instruction {
    //R instructions
    R = 0o00,
    // Memory access instructions
    LB = 0o40,
    LBU = 0o44,
    LHW = 0o41,
    LHWU = 0o45,
    LW = 0o43,
    LUI = 0o17,
    LWCz = 0o60,
    LWL = 0o42,
    LWR = 0o46,
    SB = 0o50,
    SHW = 0o51,
    SWR = 0o56,
    SWL = 0o52,
    SW = 0o53,
    SWCz = 0o70,
    // I aritmethic instructions
    ADDI = 0o10,
    ADDIU = 0o11,
    // F_DIV_IMMEDIATE = 0o13,
    // F_MUL_IMMEDIATE = 0o07,
    ANDI = 0o14,
    ORI = 0o15,
    XORI = 0o16,
    SLTI = 0o12,
    SLTIU = 0o13,
    // Jump/Branch instructions
    BEQ = 0o04,
    BLEZ = 0o06,
    BNE = 0o05,
    REGIMM = 0o01,
    J = 0o02,
    JAL = 0o03,
    JR = 0o35,
    BGTZ = 0o07,
    // F_ADD_IMMEDIATE = 0o37,
    // F_SUB_IMMEDIATE = 0o40,
    COPz = 0o20,
}

#[derive(FromPrimitive)]
enum Function {
    ADD = 0o40,
    SUB = 0o42,
    ADDU = 0o41,
    SUBU = 0o43,
    AND = 0o44,
    OR = 0o45,
    XOR = 0o46,
    SLL = 0o00,
    SLLV = 0o04,
    NOR = 0o47,
    MULT = 0o30,
    MULTU = 0o31,
    DIV = 0o32,
    DIVU = 0o33,
    SLT = 0o52,
    SLTU = 0o53,
    SRA = 0o03,
    SRAV = 0o07,
    SRL = 0o02,
    SRLV = 0o06,
    SYSCALL = 0o14,
    BREAK = 0o15,
    JALR = 0o11,
    JR = 0o10,
    MFHI = 0o20,
    MFLO = 0o22,
    MTHI = 0o21,
    MTLO = 0o23,
}

#[allow(dead_code)]
enum Branches {
    BLTZ = 0b00000,
    BLTZAL = 0b10000,
    BGEZ = 0b00001,
    BGEZAL = 0b10001,
}