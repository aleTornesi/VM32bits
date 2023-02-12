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
        for (i, v) in self.memory.getWord(self.pc).into_iter().enumerate() {
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
            Instruction::SUBI => {
                let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                let rs_value = i32::from_be_bytes(self.registers[rs as usize].to_be_bytes());
                let immediate_signed_interpretation = i32::from_be_bytes(immediate.to_be_bytes());
                let result = rs_value - immediate_signed_interpretation;
                self.registers[rt as usize] = u32::from_be_bytes(result.to_be_bytes());
            },
            Instruction::SUBIU => {
                let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                let rs_value = self.registers[rs as usize];
                let result = rs_value - immediate;
                self.registers[rt as usize] = result;
            },
            Instruction::LB => {
                let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize] as i32;
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let signed_content = i8::from_be_bytes(self.memory.getByte((index + offset) as usize));
                self.registers[rt as usize] =  u32::from_be_bytes((signed_content as i32).to_be_bytes());
            },
            Instruction::LBU => {
                let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize] as i32;
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let content = u8::from_be_bytes(self.memory.getByte((index + offset) as usize));
                self.registers[rt as usize] = content as u32;
            },
            Instruction::LHW => {
                let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize] as i32;
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let signed_content = i16::from_be_bytes(self.memory.getHalfWord((index + offset) as usize));
                self.registers[rt as usize] =  u32::from_be_bytes((signed_content as i32).to_be_bytes());
            },
            Instruction::LHWU => {
                let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize] as i32;
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let content = u16::from_be_bytes(self.memory.getHalfWord((index + offset) as usize));
                self.registers[rt as usize] = content as u32;
            },
            Instruction::LW => {
                let (rs, rt, immediate) = CPU::get_r_immediate_instructions_values(instruction);
                let index = self.registers[rs as usize] as i32;
                let offset = i32::from_be_bytes(immediate.to_be_bytes());
                let signed_content = i32::from_be_bytes(self.memory.getWord((index + offset) as usize));
                self.registers[rt as usize] =  u32::from_be_bytes((signed_content as i32).to_be_bytes());
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



struct Instruction{}
impl Instruction {
    //R instructions
    const R: u8 = 0o00;
    // Memory access instructions
    const LB: u8 = 0o40;
    const LBU: u8 = 0o44;
    const LHW: u8 = 0o41;
    const LHWU: u8 = 0o45;
    const LUI: u8 = 0o17;
    const LW: u8 = 0o43;
    const LWCz: u8 = 0o60;
    const LWL: u8 = 0o42;
    const LWR: u8 = 0o46;
    const SB: u8 = 0o50;
    const SHW: u8 = 0o51;
    const SWR: u8 = 0o56;
    const SWL: u8 = 0o52;
    const SW: u8 = 0o53;
    const MTHI: u8 = 0o21;
    const MTLO: u8 = 0o22;
    const MFHI: u8 = 0o23;
    const SWCz: u8 = 0o70;
    const MFLO: u8 = 0o24;
    // I aritmethic instructions
    const ADDI: u8 = 0o10;
    const ADDIU: u8 = 0o11;
    const SUBI: u8 = 0o05;
    const SUBIU: u8 = 0o06;
    const F_DIV_IMMEDIATE: u8 = 0o13;
    const F_MUL_IMMEDIATE: u8 = 0o07;
    const ANDI: u8 = 0o14;
    const ORI: u8 = 0o15;
    const XORI: u8 = 0o16;
    const SLL: u8 = 0o17;
    const SRL: u8 = 0o20;
    const SLTI: u8 = 0o12;
    const SLTIU: u8 = 0o13;
    // Jump/Branch instructions
    const BEQ: u8 = 0o04;
    const BLEZ: u8 = 0o06;
    const BNE: u8 = 0o05;
    const REGIMM:u8 = 0o01;
    const J: u8 = 0o02;
    const JAL: u8 = 0o03;
    const JR: u8 = 0o35;
    const BGTZ: u8 = 0o07;
    const F_ADD_IMMEDIATE: u8 = 0o37;
    const F_SUB_IMMEDIATE: u8 = 0o40;
    const COPz: u8 = 0o20;
}

struct Function{}
impl Function {
    const ADD: u8 = 0o40;
    const SUB: u8 = 0o42;
    const ADDU: u8 = 0o41;
    const SUBU: u8 = 0o43;
    const AND: u8 = 0o44;
    const OR: u8 = 0o45;
    const XOR: u8 = 0o46;
    const SLL: u8 = 0o00;
    const SLLV: u8 = 0o04;
    const NOR: u8 = 0o47;
    const MULT: u8 = 0o30;
    const MULTU: u8 = 0o31;
    const DIV: u8 = 0o32;
    const DIVU: u8 = 0o33;
    const SLT: u8 = 0o52;
    const SLTU: u8 = 0o53;
    const SRA: u8 = 0o03;
    const SRAV: u8 = 0o07;
    const SRL: u8 = 0o02;
    const SRLV: u8 = 0o06;
    const SYSCALL: u8 = 0o14;
    const BREAK: u8 = 0o15;
    const JALR: u8 = 0o11;
    const JR: u8 = 0o10;
    const MFHI: u8 = 0o20;
    const MFLO: u8 = 0o22;
    const MTHI: u8 = 0o21;
    const MTLO: u8 = 0o23;
}

struct Branches{}
impl Branches {
    const BLTZ: u8 = 0b00000;
    const BLTZAL: u8 = 0b10000;
    const BGEZ: u8 = 0b00001;
    const BGEZAL: u8 = 0b10001;
}