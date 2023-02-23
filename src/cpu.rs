extern crate num;

use num_derive::FromPrimitive;

use crate::memory_mapper::MemoryMapper;

pub struct CPU<'a> {
    registers: [u32; 32],
    pc: u32,
    hi: u32,
    lo: u32,
    memory_mapper: &'a mut MemoryMapper
}

impl<'a> CPU<'a> {
    const REGISTER_MASK: u32 = 0b00000011111;
    const FUNCTION_MASK: u32 = 0x0000003f;
    const IMMEDIATE_MASK: u32 = 0x0000ffff;

    pub fn new(memory_mapper:  &'a mut MemoryMapper) -> Self {
        CPU{ registers: [0; 32], pc: 0, hi: 0, lo: 0, memory_mapper }
    }

    fn fetch(&mut self) -> u32 {
        let instruction_bytes:[u8; 4] = self.memory_mapper.get_word(self.pc as u32);
        let res = u32::from_be_bytes(instruction_bytes);
        self.pc += 4;
        return res;
    }

    #[cfg(test)]
    pub fn get_register_value(&self, i: usize) -> u32 {
        return self.registers[i];
    }

    fn immediate_unsigned_op_write_r(&mut self, instruction: u32, op: fn(u32, u32) -> u32) {
        let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
        let rs_value = self.registers[rs as usize];
        self.registers[rt as usize] = op(rs_value, immediate);
    }

    fn immediate_signed_op_write_r(&mut self, instruction: u32, op: fn(i32, i32) -> i32) {
        let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
        let rs_value = u32_to_i32_interpreatation(self.registers[rs as usize]);
        let immediate = u32_to_i32_interpreatation(immediate);
        let result = op(rs_value, immediate);
        self.registers[rt as usize] = i32_interpreatation_to_u32(result);
    }


    fn load(&mut self, instruction: u32, op: fn(&mut MemoryMapper, u32) -> u32) {
        let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
        let index = self.registers[rs as usize];
        let offset = u32_to_i32_interpreatation(immediate);
        self.registers[rt as usize] = op(self.memory_mapper, CPU::calculate_address_offset(index, offset))
    }

    fn calculate_address_offset(address: u32, offset: i32) -> u32 {
        return (address as isize + offset as isize) as u32;
    }

    fn store(&mut self, instruction: u32, op: fn(&mut MemoryMapper, u32, u32)) {
        let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
        let address = self.registers[rs as usize];
        let offset = u32_to_i32_interpreatation(immediate);
        let signed_content = self.registers[rt as usize];
        op(self.memory_mapper, signed_content, CPU::calculate_address_offset(address, offset));
    }

    fn branch_instruction(&mut self, instruction: u32, condition: fn(u32, u32) -> bool) {
        let (rs, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
        let offset = u32_to_i32_interpreatation(immediate);
        if condition(self.registers[rs as usize], self.registers[rt as usize]) {
            self.branch(offset)
        }
    }

    fn branch_instruction_signed_values(&mut self, instruction: u32, condition: fn(i32) -> bool) {
        let (rs, _, immediate) = CPU::get_immediate_instructions_values(instruction);
        let offset = u32_to_i32_interpreatation(immediate);
        let rs_value = u32_to_i32_interpreatation(self.registers[rs as usize]);
        if condition(rs_value) {
            self.branch(offset)
        }
    }

    fn execute(&mut self, instruction: u32) -> bool {
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
            Instruction::ADDI => self.immediate_signed_op_write_r(instruction, |rs, immediate| rs + immediate),
            Instruction::ADDIU => self.immediate_unsigned_op_write_r(instruction, |rs, immediate| rs + immediate),
            Instruction::LB => self.load(instruction, |mm, address| i32_interpreatation_to_u32(i8::from_be_bytes(mm.get_byte(address)) as i32)),
            Instruction::LBU => self.load(instruction, |mm, address| u8::from_be_bytes(mm.get_byte(address)) as u32),
            Instruction::LHW => self.load(instruction, |mm, address| i32_interpreatation_to_u32(i16::from_be_bytes(mm.get_half_word(address)) as i32)),
            Instruction::LHWU => self.load(instruction, |mm, address| u16::from_be_bytes(mm.get_half_word(address)) as u32),
            Instruction::LW => self.load(instruction, |mm, address| u32::from_be_bytes(mm.get_word(address))),
            Instruction::LUI => {
                let (_, rt, immediate) = CPU::get_immediate_instructions_values(instruction);
                self.registers[rt as usize] = immediate << 16;
            },
            Instruction::LWC1 => todo!(),
            Instruction::LWL => todo!(),
            Instruction::LWR => todo!(),             // let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
            Instruction::SB => self.store(instruction, |mm, address, value| mm.write_byte(address, (value as u8).to_be_bytes())),
            Instruction::SHW => self.store(instruction, |mm, address, value| mm.write_half_word(address, (value as u16).to_be_bytes())),
            Instruction::SW => self.store(instruction, |mm, address, value| mm.write_word(address, value.to_be_bytes())),
            Instruction::SWR => todo!(),
            Instruction::SWL => todo!(),
            Instruction::SWC1 => todo!(),
            Instruction::ANDI => self.immediate_unsigned_op_write_r(instruction, |rs, immediate| rs & immediate),
            Instruction::ORI => self.immediate_unsigned_op_write_r(instruction, |rs, immediate| rs | immediate),
            Instruction::XORI => self.immediate_unsigned_op_write_r(instruction, |rs, immediate| rs ^ immediate),
            Instruction::SLTI => self.immediate_signed_op_write_r(instruction, |rs, immediate| (rs < immediate) as i32),
            Instruction::SLTIU => self.immediate_unsigned_op_write_r(instruction, |rs, immediate| (rs < immediate) as u32),
            Instruction::BEQ => self.branch_instruction(instruction, |rs, rt| rs == rt),
            Instruction::BNE => self.branch_instruction(instruction, |rs, rt| rs != rt),
            Instruction::BLEZ => self.branch_instruction_signed_values(instruction, |rs| rs <= 0),
            Instruction::BGTZ => self.branch_instruction_signed_values(instruction, |rs| rs >= 0),
            Instruction::REGIMM => self.regimm_branching(instruction),
            Instruction::J => self.pc = self.get_jump_address(instruction),
            Instruction::JAL => {
                self.registers[31] = (self.pc + 4) as u32;
                self.pc = self.get_jump_address(instruction);
            },
            Instruction::COP1 => todo!(),


        }
        return false;
    }

    fn branch_al_instruction(&mut self, instruction: u32, condition: fn(i32) -> bool) {
        self.registers[31] = self.pc + 4;
        self.branch_instruction_signed_values(instruction, condition);
    }

    fn regimm_branching(&mut self, instruction: u32) {
        let branch: u8 = ((instruction >> 16) & CPU::REGISTER_MASK) as u8;
        let branch: Branch = num::FromPrimitive::from_u8(branch).unwrap();
        match branch {
            Branch::BLTZ => self.branch_instruction_signed_values(instruction, |rs| rs < 0),
            Branch::BLTZAL => self.branch_al_instruction(instruction, |rs| rs < 0),
            Branch::BGEZ => self.branch_instruction_signed_values(instruction, |rs| rs > 0),
            Branch::BGEZAL => self.branch_al_instruction(instruction, |rs| rs > 0),
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

    fn branch(&mut self, offset: i32) {
        let mut pc_content = self.pc as i32;
        pc_content += offset;
        self.pc = pc_content as u32;
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
                self.lo = u64::from_be_bytes(result.to_be_bytes()) as u32;
                self.hi = u64::from_be_bytes((result >> 32).to_be_bytes()) as u32;
            },
            Function::MULTU => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                let result = (rs_value * rt_value) as u64;
                self.lo = result as u32;
                self.hi = (result >> 32) as u32;
            },
            Function::DIV => {
                let rs_value = i32::from_be_bytes(self.registers[rs as usize].to_be_bytes());
                let rt_value = i32::from_be_bytes(self.registers[rt as usize].to_be_bytes());
                self.lo = u32::from_be_bytes((rs_value / rt_value).to_be_bytes());
                self.hi = u32::from_be_bytes((rs_value % rt_value).to_be_bytes());
            },
            Function::DIVU => {
                let rs_value = self.registers[rs as usize];
                let rt_value = self.registers[rt as usize];
                self.lo = rs_value / rt_value;
                self.hi = rs_value % rt_value;
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
                self.pc = self.registers[rs as usize];
            },
            Function::JR => {
                let rs_value = self.registers[rs as usize];
                self.pc = rs_value;
            },
            Function::MFHI => {
                self.registers[rd as usize] = self.hi;
            },
            Function::MFLO => {
                self.registers[rd as usize] = self.lo;
            },
            Function::MTHI => {
                self.hi = self.registers[rd as usize];
            },
            Function::MTLO => {
                self.hi = self.registers[rd as usize];
            },
            Function::SYSCALL => {
                let code = ((rs as u32) << 10) | ((rt as u32) << 15) | (shift_amount as u32);
                return match code {
                    10_u32 => true,
                    _ => false
                }
            },
            Function::MOVCI => todo!(),

        }

        return false
    }


    fn step(&mut self) -> bool {
        let instruction = self.fetch();
        return self.execute(instruction);
    }

    pub fn run(&mut self) {
        let halt = self.step();
        if !halt {
            self.run();
        }
    }
}


#[derive(FromPrimitive)]
pub enum Instruction {
    //R instructions
    R = 0o00,
    // Memory access instructions
    LB = 0o40,
    LBU = 0o44,
    LHW = 0o41,
    LHWU = 0o45,
    LW = 0o43,
    LUI = 0o17,
    LWC1 = 0o61,
    LWL = 0o42,
    LWR = 0o46,
    SB = 0o50,
    SHW = 0o51,
    SWR = 0o56,
    SWL = 0o52,
    SW = 0o53,
    SWC1 = 0o70,
    // I aritmethic instructions
    ADDI = 0o10,
    ADDIU = 0o11,
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
    COP1 = 0o21,
}

#[derive(FromPrimitive)]
pub enum Function {
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
    MOVCI = 0o01
}

#[derive(FromPrimitive)]
pub enum Branch {
    BLTZ = 0b00000,
    BLTZAL = 0b10000,
    BGEZ = 0b00001,
    BGEZAL = 0b10001,
}

pub fn u32_to_i32_interpreatation(value: u32) -> i32 {
    return i32::from_be_bytes(value.to_be_bytes());
}

pub fn i32_interpreatation_to_u32(value: i32) -> u32 {
    return u32::from_be_bytes(value.to_be_bytes());
}
