extern crate num;

use num_derive::FromPrimitive;

use crate::memory::Memory;

pub struct CPU {
    registers: [u32; 32],
    f_registers: [f32; 32],
    memory: Memory,
    pc: usize,
    HI: u32,
    LO: u32
    //index: u32
}

impl CPU {
    const REGISTER_MASK: u32 = 0b00000011111;
    const FUNCTION_MASK: u32 = 0x0000003f;
    const IMMEDIATE_MASK: u32 = 0x0000ffff;

    pub fn new(memory: Memory) -> CPU {
        CPU{ registers: [0; 32], f_registers: [0_f32; 32], memory, pc: 0, HI: 0, LO: 0 }
    }

    fn fetch(&mut self) -> u32 {
        let mut instructionBytes:[u8; 4] = [0; 4];
        for (i, v) in self.memory.get_word(self.pc).into_iter().enumerate() {
            instructionBytes[i] = v;
        }
        let res = u32::from_be_bytes(instructionBytes);
        return res;
    }

    #[cfg(test)]
    pub fn get_register_value(&self, i: usize) -> u32 {
        return self.registers[i];
    }

    pub fn execute(&mut self, instruction: u32) -> bool {
        let op_code: u8 = (instruction >> 26) as u8;
        let op_code: Instruction = num::FromPrimitive::from_u8(op_code).unwrap();
        match op_code {
            Instruction::R => {
                let rs:u8 = ((instruction >> 21) & CPU::REGISTER_MASK) as u8;
                let rt:u8 = ((instruction >> 16) & CPU::REGISTER_MASK) as u8;
                let rd:u8 = ((instruction >> 11) & CPU::REGISTER_MASK) as u8;
                let shift_amount: u8 = ((instruction >> 6) & CPU::REGISTER_MASK) as u8;
                let function: u8 = (instruction & CPU::FUNCTION_MASK) as u8;
                return self.alu_operation(rs, rt, rd, shift_amount, function);
            },
            Instruction::ADDI => {
                let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                let rs_value = i32::from_be_bytes(self.registers[rs as usize].to_be_bytes());
                let immediate_signed_interpretation = i32::from_be_bytes(immediate.to_be_bytes());
                let result = rs_value + immediate_signed_interpretation;
                self.registers[rt as usize] = u32::from_be_bytes(result.to_be_bytes());
            },
            Instruction::ADDIU => {
                let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                let rs_value = self.registers[rs as usize];
                let result = rs_value + immediate;
                self.registers[rt as usize] = result;
            },
            Instruction::LB => {
                let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize];
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let signed_content = i8::from_be_bytes(self.memory.get_byte((index as i32 + offset) as usize));
                self.registers[rt as usize] =  u32::from_be_bytes((signed_content as i32).to_be_bytes());
            },
            Instruction::LBU => {
                let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize] as i32;
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let content = u8::from_be_bytes(self.memory.get_byte((index + offset) as usize));
                self.registers[rt as usize] = content as u32;
            },
            Instruction::LHW => {
                let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize] as i32;
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let signed_content = i16::from_be_bytes(self.memory.get_half_word((index + offset) as usize));
                self.registers[rt as usize] =  u32::from_be_bytes((signed_content as i32).to_be_bytes());
            },
            Instruction::LHWU => {
                let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize] as i32;
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let content = u16::from_be_bytes(self.memory.get_half_word((index + offset) as usize));
                self.registers[rt as usize] = content as u32;
            },
            Instruction::LW => {
                let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize];
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let signed_content = i32::from_be_bytes(self.memory.get_word((index as i32 + offset) as usize));
                self.registers[rt as usize] =  u32::from_be_bytes((signed_content as i32).to_be_bytes());
            },
            Instruction::LUI => {
                let (_, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
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
                let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize];
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let signed_content = self.registers[rt as usize] as u8;
                self.memory.write_byte((index as i32 + offset) as usize, signed_content);
            },
            Instruction::SHW => {
                let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize];
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let signed_content = self.registers[rt as usize] as u16;
                self.memory.write_half_word((index as i32 + offset) as usize, signed_content.to_be_bytes());
            },
            Instruction::SW => {
                let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize];
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let signed_content = self.registers[rt as usize];
                self.memory.write_word((index as i32 + offset) as usize, signed_content.to_be_bytes());
            },
            Instruction::SWR => todo!(),
            Instruction::SWL => todo!(),
            Instruction::SWCz => todo!(),
            Instruction::ANDI => {
                let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                let rs_content = self.registers[rs as usize];
                let result = rs_content & immediate;
                self.registers[rt as usize] = result;
            },
            Instruction::ORI => {
                let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                let rs_content = self.registers[rs as usize];
                let result = rs_content | immediate;
                self.registers[rt as usize] = result;
            },
            Instruction::XORI => {
                let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                let rs_content = self.registers[rs as usize];
                let result = rs_content & immediate;
                self.registers[rt as usize] = result;
            },
            Instruction::SLTI => {
                let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                let immediate = i32::from_be_bytes(immediate.to_be_bytes());
                let rs_content = i32::from_be_bytes(self.registers[rs as usize].to_be_bytes());
                let result = (rs_content < immediate) as u32;
                self.registers[rt as usize] = result;
            },
            Instruction::SLTIU => {
                let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                let rs_content = self.registers[rs as usize];
                let result = (rs_content < immediate) as u32;
                self.registers[rt as usize] = result;
            },
            Instruction::BEQ => {
                let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                if self.registers[rs as usize] == self.registers[rt as usize] {
                    self.branch(offset as isize)
                }
            },
            Instruction::BNE => {
                let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                if self.registers[rs as usize] != self.registers[rt as usize] {
                    self.branch(offset as isize)
                }
            },
            Instruction::BLEZ => {
                let (rs, _, immediate) = CPU::get_immediate_instructions_values(instruction);
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                if self.registers[rs as usize] <= 0 {
                    self.branch(offset as isize)
                }
            },
            Instruction::BGTZ => {
                let (rs, _, immediate) = CPU::get_immediate_instructions_values(instruction);
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                if self.registers[rs as usize] > 0 {
                    self.branch(offset as isize)
                }
            },
            Instruction::REGIMM => {
                let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                self.regimm_branching(rs, rt, immediate);
            },
            Instruction::J => {
                let address = self.get_jump_address(instruction);
                self.pc = address as usize;
            },
            Instruction::JAL => {
                self.registers[31] = (self.pc + 4) as u32;
                self.pc = self.get_jump_address(instruction) as usize;
            },
            Instruction::COPz => todo!(),

            
        }
        return true;
    }

    fn regimm_branching(&mut self, rs:u8, instruction: u8, offset: u32) {
        let branch: Branch = num::FromPrimitive::from_u8(instruction).unwrap();
        let signed_rs_content = i32::from_be_bytes(self.registers[rs as usize].to_be_bytes());
        match branch {
            Branch::BLTZ => {
                if signed_rs_content < 0 {
                    self.branch(offset as isize);
                }
            },
            Branch::BLTZAL => {
                self.registers[31] = (self.pc + 4) as u32;
                if signed_rs_content < 0 {
                    self.branch(offset as isize);
                }
            },
            Branch::BGEZ => {
                if signed_rs_content > 0 {
                    self.branch(offset as isize);
                }
            },
            Branch::BGEZAL => {
                self.registers[31] = (self.pc + 4) as u32;
                if signed_rs_content > 0 {
                    self.branch(offset as isize);
                }
            },
        }
    }

    fn get_immediate_instructions_values(instruction: u32) -> (u8, u8, u32) {
        let rs = ((instruction >> 21) & CPU::REGISTER_MASK) as u8;
        let rd = ((instruction >> 16) & CPU::REGISTER_MASK) as u8;
        let immediate = (instruction & CPU::IMMEDIATE_MASK) as u32;
        return (rs, rd, immediate);
    }

    fn get_jump_address(&self, instruction: u32) -> u32 {
        let pseudo_address = instruction & 0b0000_0011_1111_1111_1111_1111_1111_1111;
        return pseudo_address << 2 + (self.pc & 0xf0000000);
    }

    fn branch(&mut self, offset: isize) {
        let mut pc_content = self.pc as isize;
        pc_content += offset;
        self.pc = pc_content as usize;
    }

    fn alu_operation(&mut self, rs:u8, rt:u8, rd:u8, shift_amount: u8, function: u8) -> bool {
        
        let function: Function = num::FromPrimitive::from_u8(function).unwrap();
        if rd == 0 && !matches!(function, Function::SYSCALL)  {
            panic!("You cannot write on the zero register");
        }
        match function {
            Function::ADD => {
                let rs_value = i32::from_be_bytes(self.registers[rs as usize].to_be_bytes());
                let rt_value = i32::from_be_bytes(self.registers[rt as usize].to_be_bytes());
                let result: i32 = rs_value + rt_value;
                self.registers[rd as usize] =  u32::from_be_bytes(result.to_be_bytes())
            },
            Function::ADDU => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = rs_value + rt_value;
                self.registers[rd as usize] = result;
            },
            Function::SUB => {
                let rs_value = i32::from_be_bytes(self.registers[rs as usize].to_be_bytes());
                let rt_value = i32::from_be_bytes(self.registers[rt as usize].to_be_bytes());
                let result: i32 = rs_value - rt_value;
                self.registers[rd as usize] =  u32::from_be_bytes(result.to_be_bytes())
            },
            Function::SUBU => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = rs_value - rt_value;
                self.registers[rd as usize] = result;
            },
            Function::MULT => {
                let rs_value = i32::from_be_bytes(self.registers[rs as usize].to_be_bytes());
                let rt_value = i32::from_be_bytes(self.registers[rt as usize].to_be_bytes());
                let result = (rs_value * rt_value) as i64;
                self.LO = u64::from_be_bytes(result.to_be_bytes()) as u32;
                self.HI = u64::from_be_bytes((result >> 32).to_be_bytes()) as u32;
            },
            Function::MULTU => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = (rs_value * rt_value) as u64;
                self.LO = result as u32;
                self.HI = (result >> 32) as u32;
            },
            Function::DIV => {
                let rs_value = i32::from_be_bytes(self.registers[rs as usize].to_be_bytes());
                let rt_value = i32::from_be_bytes(self.registers[rt as usize].to_be_bytes());
                self.LO = u32::from_be_bytes((rs_value / rt_value).to_be_bytes());
                self.HI = u32::from_be_bytes((rs_value % rt_value).to_be_bytes());
            },
            Function::DIVU => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                self.LO = rs_value / rt_value;
                self.HI = rs_value % rt_value;
            },
            Function::AND => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = rs_value & rt_value;
                self.registers[rd as usize] = result;
            },
            Function::OR => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = rs_value | rt_value;
                self.registers[rd as usize] = result;
            },
            Function::XOR => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = rs_value ^ rt_value;
                self.registers[rd as usize] = result;
            },
            Function::NOR => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = !(rs_value | rt_value);
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
            Function::BREAK => todo!(),
            Function::JALR => {
                self.registers[rd as usize];
                self.pc = self.registers[rs as usize] as usize;
            },
            Function::JR => {
                let rs_value = self.registers[rs as usize];
                self.pc = rs_value as usize;
            },
            Function::MFHI => {
                self.registers[rd as usize] = self.HI;
            },
            Function::MFLO => {
                self.registers[rd as usize] = self.LO;
            },
            Function::MTHI => {
                self.HI = self.registers[rd as usize];
            },
            Function::MTLO => {
                self.HI = self.registers[rd as usize];
            },
            Function::SYSCALL => {
                let code = ((rs as u32) << 10) | ((rt as u32) << 15) | (shift_amount as u32);
                println!("{}", code);
                return match code {
                    10_u32 => false,
                    _ => true
                }
            },
            
        }

        return true
    }
    

    fn step(&mut self) -> bool {
        let instruction = self.fetch();
        return self.execute(instruction);
    }

    fn run(&mut self) {
        let halt = self.step();

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
    BREAK = 0o15,
    JALR = 0o11,
    JR = 0o10,
    MFHI = 0o20,
    MFLO = 0o22,
    SYSCALL = 0o14,
    MTHI = 0o21,
    MTLO = 0o23,
}

#[derive(FromPrimitive)]
enum Branch {
    BLTZ = 0b00000,
    BLTZAL = 0b10000,
    BGEZ = 0b00001,
    BGEZAL = 0b10001,
}