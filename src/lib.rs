pub mod cpu;
pub mod memory;

#[cfg(test)]
mod tests {
    use crate::cpu::CPU;
    use crate::memory::Memory;

    #[test]
    fn ADD() {
        let mut cpu = CPU::new(Memory::new(0));

        let op_code = 0o10_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 20_u32;
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);

        let op_code = 0o10_u32;
        let rs = 5_u32;
        let rd = 3_u32;
        let immediate = 10_u32;
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);

        let op_code = 0o00_u32;
        let rs = 3_u32;
        let rt = 10_u32;
        let rd = 5_u32;
        let shift_amount = 0_u32;
        let function = 0o40_u32;
        let instruction = form_r_instruction(op_code, rs, rt, rd, shift_amount, function);
        cpu.execute(instruction);

        assert_eq!(i32::from_be_bytes(cpu.get_register_value(5).to_be_bytes()), 30_i32)
    }
    
    #[test]
    fn ADDU() {
        let mut cpu = CPU::new(Memory::new(0));

        let op_code = 0o10_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 20_u32;
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);

        let op_code = 0o10_u32;
        let rs = 5_u32;
        let rd = 3_u32;
        let immediate = 10_u32;
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);

        let op_code = 0o00_u32;
        let rs = 3_u32;
        let rt = 10_u32;
        let rd = 5_u32;
        let shift_amount = 0_u32;
        let function = 0o41_u32;
        let instruction = form_r_instruction(op_code, rs, rt, rd, shift_amount, function);
        cpu.execute(instruction);

        assert_eq!(cpu.get_register_value(5), 30)
    }
    
    #[test]
    fn SUB() {
        let mut cpu = CPU::new(Memory::new(0));

        let op_code = 0o10_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 20_u32;
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);

        let op_code = 0o10_u32;
        let rs = 5_u32;
        let rd = 3_u32;
        let immediate = 10_u32;
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);

        let op_code = 0o00_u32;
        let rs = 10_u32;
        let rt = 3_u32;
        let rd = 5_u32;
        let shift_amount = 0_u32;
        let function = 0o42_u32;
        let instruction = form_r_instruction(op_code, rs, rt, rd, shift_amount, function);
        cpu.execute(instruction);

        assert_eq!(i32::from_be_bytes(cpu.get_register_value(5).to_be_bytes()), 10_i32)
    }
    
    #[test]
    fn SUBU() {
        let mut cpu = CPU::new(Memory::new(0));

        let op_code = 0o10_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 20_u32;
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);

        let op_code = 0o10_u32;
        let rs = 5_u32;
        let rd = 3_u32;
        let immediate = 10_u32;
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);

        let op_code = 0o00_u32;
        let rs = 10_u32;
        let rt = 3_u32;
        let rd = 5_u32;
        let shift_amount = 0_u32;
        let function = 0o43_u32;
        let instruction = form_r_instruction(op_code, rs, rt, rd, shift_amount, function);
        cpu.execute(instruction);

        assert_eq!(cpu.get_register_value(5), 10)
    }
    
    #[test]
    fn MULT() {
        let mut cpu = CPU::new(Memory::new(0));

        let op_code = 0o10_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 20_u32;
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);

        let op_code = 0o10_u32;
        let rs = 5_u32;
        let rd = 3_u32;
        let immediate = 10_u32;
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);

        let op_code = 0o00_u32;
        let rs = 3_u32;
        let rt = 10_u32;
        let rd = 5_u32;
        let shift_amount = 0_u32;
        let function = 0o30_u32;
        let instruction = form_r_instruction(op_code, rs, rt, rd, shift_amount, function);
        cpu.execute(instruction);

        assert_eq!(i32::from_be_bytes(cpu.get_register_value(5).to_be_bytes()), 200_i32)
    }
    
    #[test]
    fn MULTU() {
        let mut cpu = CPU::new(Memory::new(0));

        let op_code = 0o10_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 20_u32;
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);

        let op_code = 0o10_u32;
        let rs = 5_u32;
        let rd = 3_u32;
        let immediate = 10_u32;
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);

        let op_code = 0o00_u32;
        let rs = 3_u32;
        let rt = 10_u32;
        let rd = 5_u32;
        let shift_amount = 0_u32;
        let function = 0o31_u32;
        let instruction = form_r_instruction(op_code, rs, rt, rd, shift_amount, function);
        cpu.execute(instruction);

        assert_eq!(cpu.get_register_value(5), 200)
    }
    
    #[test]
    fn DIV() {
        let mut cpu = CPU::new(Memory::new(0));

        let op_code = 0o10_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 20_u32;
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);

        let op_code = 0o10_u32;
        let rs = 5_u32;
        let rd = 3_u32;
        let immediate = 10_u32;
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);

        let op_code = 0o00_u32;
        let rs = 10_u32;
        let rt = 3_u32;
        let rd = 5_u32;
        let shift_amount = 0_u32;
        let function = 0o32_u32;
        let instruction = form_r_instruction(op_code, rs, rt, rd, shift_amount, function);
        cpu.execute(instruction);

        assert_eq!(i32::from_be_bytes(cpu.get_register_value(5).to_be_bytes()), 2_i32)
    }
    
    #[test]
    fn DIVU() {
        let mut cpu = CPU::new(Memory::new(0));

        let op_code = 0o10_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 20_u32;
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);

        let op_code = 0o10_u32;
        let rs = 5_u32;
        let rd = 3_u32;
        let immediate = 10_u32;
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);

        let op_code = 0o00_u32;
        let rs = 10_u32;
        let rt = 3_u32;
        let rd = 5_u32;
        let shift_amount = 0_u32;
        let function = 0o33_u32;
        let instruction = form_r_instruction(op_code, rs, rt, rd, shift_amount, function);
        cpu.execute(instruction);

        assert_eq!(cpu.get_register_value(5), 2)
    }

    
    #[test]
    fn ADD_IMMEDIATE() {
        let mut cpu = CPU::new(Memory::new(0));
        
        
        let op_code = 0o10_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 210_u32;
        let og_rs_value = cpu.get_register_value(rs as usize);
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);
        let rd_value = cpu.get_register_value(rd as usize);
        assert_eq!(i32::from_be_bytes(rd_value.to_be_bytes()), i32::from_be_bytes(og_rs_value.to_be_bytes()) + i32::from_be_bytes(immediate.to_be_bytes()))
    }
    #[test]
    fn ADD_IMMEDIATE_UNSIGNED() {
        let mut cpu = CPU::new(Memory::new(0));
        
        
        let op_code = 0o11_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 210_u32;
        let og_rs_value = cpu.get_register_value(rs as usize);
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);
        let rd_value = cpu.get_register_value(rd as usize);
        assert_eq!(rd_value, og_rs_value + immediate)
    }
    
    #[test]
    fn SUB_IMMEDIATE() {
        let mut cpu = CPU::new(Memory::new(0));

        
        let op_code = 0o05_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 210_u32;
        let og_rs_value = cpu.get_register_value(rs as usize);
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);
        let rd_value = cpu.get_register_value(rd as usize);
        assert_eq!(i32::from_be_bytes(rd_value.to_be_bytes()), i32::from_be_bytes(og_rs_value.to_be_bytes()) - i32::from_be_bytes(immediate.to_be_bytes()))
    }
    #[test]
    fn SUB_IMMEDIATE_UNSIGNED() {
        let mut cpu = CPU::new(Memory::new(0));
        
        let op_code = 0o11_u32;
        let rs = 2_u32;
        let rd = 5_u32;
        let immediate = 210_u32;
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);
        
        let op_code = 0o06_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 200_u32;
        let og_rs_value = cpu.get_register_value(rs as usize);
        let instruction = form_i_instruction(op_code, rs, rd, immediate);
        cpu.execute(instruction);
        let rd_value = cpu.get_register_value(rd as usize);
        assert_eq!(rd_value, og_rs_value - immediate)
    }
    
    #[test]
    fn LB() {
        let mut cpu = CPU::new(Memory::new(1));
        
        let op_code = 0o11_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 210_u32;
        
        cpu.execute(form_i_instruction(op_code, rs, rd, immediate));
        
        if cpu.get_register_value(rd as usize) != 210 {
            assert!(false);
        }
        
        let op_code = 0o40_u32;
        let rs = 0_u32;
        let rd = 10_u32;
        let immediate = 0_u32;
        
        cpu.execute(form_i_instruction(op_code, rs, rd, immediate));
        
        assert_eq!(cpu.get_register_value(rd as usize), 0);
    }
    
    #[test]
    fn LBU() {
        let mut cpu = CPU::new(Memory::new(1));
        
        let op_code = 0o11_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 210_u32;
        
        cpu.execute(form_i_instruction(op_code, rs, rd, immediate));
        
        if cpu.get_register_value(rd as usize) != 210 {
            assert!(false);
        }
        
        let op_code = 0o44_u32;
        let rs = 0_u32;
        let rd = 10_u32;
        let immediate = 0_u32;
        
        cpu.execute(form_i_instruction(op_code, rs, rd, immediate));
        
        assert_eq!(cpu.get_register_value(rd as usize), 0);
    }
    
    #[test]
    fn LHW() {
        let mut cpu = CPU::new(Memory::new(2));
        
        let op_code = 0o11_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 210_u32;
        
        cpu.execute(form_i_instruction(op_code, rs, rd, immediate));
        
        if cpu.get_register_value(rd as usize) != 210 {
            assert!(false);
        }
        
        let op_code = 0o41_u32;
        let rs = 0_u32;
        let rd = 10_u32;
        let immediate = 0_u32;
        
        cpu.execute(form_i_instruction(op_code, rs, rd, immediate));
        
        assert_eq!(cpu.get_register_value(rd as usize), 0);
    }
    
    #[test]
    fn LHWU() {
        let mut cpu = CPU::new(Memory::new(2));
        
        let op_code = 0o11_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 210_u32;
        
        cpu.execute(form_i_instruction(op_code, rs, rd, immediate));
        
        if cpu.get_register_value(rd as usize) != 210 {
            assert!(false);
        }
        
        let op_code = 0o45_u32;
        let rs = 0_u32;
        let rd = 10_u32;
        let immediate = 0_u32;
        
        cpu.execute(form_i_instruction(op_code, rs, rd, immediate));
        
        assert_eq!(cpu.get_register_value(rd as usize), 0);
    }
    
    #[test]
    fn LW() {
        let mut cpu = CPU::new(Memory::new(4));
        
        let op_code = 0o11_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 210_u32;
        
        cpu.execute(form_i_instruction(op_code, rs, rd, immediate));
        
        if cpu.get_register_value(rd as usize) != 210 {
            assert!(false);
        }
        
        let op_code = 0o45_u32;
        let rs = 0_u32;
        let rd = 10_u32;
        let immediate = 0_u32;
        
        cpu.execute(form_i_instruction(op_code, rs, rd, immediate));
        
        assert_eq!(cpu.get_register_value(rd as usize), 0);
    }
    
    #[test]
    fn LUI() {
        let mut cpu = CPU::new(Memory::new(0));
        
        let op_code = 0o17_u32;
        let rs = 0_u32;
        let rd: u32 = 5_u32;
        let immediate = 1_u32;
        
        cpu.execute(form_i_instruction(op_code, rs, rd, immediate));
        
        assert_eq!(cpu.get_register_value(rd as usize), immediate << 16);
    }
    
    
    fn form_i_instruction(op_code: u32, rs: u32, rd: u32, immediate: u32) -> u32 {
        return (op_code << 26) + (rs << 21) + (rd << 16) + immediate;
    }
    
    fn form_r_instruction(op_code: u32, rs: u32, rt: u32, rd: u32, shift_amount :u32, function: u32) -> u32 {
        return (op_code << 26) + (rs << 21) + (rt << 16) + (rd << 11) + (shift_amount << 6) + function;
    }
    
    

}

