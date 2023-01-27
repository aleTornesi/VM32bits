mod cpu;

use crate::cpu::CPU;
#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use crate::cpu::CPU;

    
    #[test]
    fn LW() {
        assert_eq!(true, true)
    }
    
    #[test]
    fn SW() {
        assert_eq!(true, true)
    }

    #[test]
    fn ADD_IMMEDIATE() {
        let mut cpu = CPU::new(BytesMut::new());


        let opCode = 0o03_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 210_u32;
        let og_rs_value = cpu.get_register_value(rs as u8);
        let instruction = formInstruction(opCode, rs, rd, immediate);
        cpu.execute(instruction);
        let rd_value = cpu.get_register_value(rd as u8);
        assert_eq!(i32::from_ne_bytes(rd_value.to_ne_bytes()), i32::from_ne_bytes(og_rs_value.to_ne_bytes()) + i32::from_ne_bytes(immediate.to_ne_bytes()))
    }
    #[test]
    fn ADD_IMMEDIATE_UNSIGNED() {
        let mut cpu = CPU::new(BytesMut::new());


        let opCode = 0o04_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 210_u32;
        let og_rs_value = cpu.get_register_value(rs as u8);
        let instruction = formInstruction(opCode, rs, rd, immediate);
        cpu.execute(instruction);
        let rd_value = cpu.get_register_value(rd as u8);
        assert_eq!(rd_value, og_rs_value + immediate)
    }
    
    #[test]
    fn SUB_IMMEDIATE() {
        let mut cpu = CPU::new(BytesMut::new());


        let opCode = 0o05_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 210_u32;
        let og_rs_value = cpu.get_register_value(rs as u8);
        let instruction = formInstruction(opCode, rs, rd, immediate);
        cpu.execute(instruction);
        let rd_value = cpu.get_register_value(rd as u8);
        assert_eq!(i32::from_ne_bytes(rd_value.to_ne_bytes()), i32::from_ne_bytes(og_rs_value.to_ne_bytes()) - i32::from_ne_bytes(immediate.to_ne_bytes()))
    }
    #[test]
    fn SUB_IMMEDIATE_UNSIGNED() {
        let mut cpu = CPU::new(BytesMut::new());

        let opCode = 0o04_u32;
        let rs = 2_u32;
        let rd = 5_u32;
        let immediate = 210_u32;
        let instruction = formInstruction(opCode, rs, rd, immediate);
        cpu.execute(instruction);

        let opCode = 0o06_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 200_u32;
        let og_rs_value = cpu.get_register_value(rs as u8);
        let instruction = formInstruction(opCode, rs, rd, immediate);
        cpu.execute(instruction);
        let rd_value = cpu.get_register_value(rd as u8);
        assert_eq!(rd_value, og_rs_value - immediate)
    }
    
    #[test]
    fn MUL_IMMEDIATE() {
        let mut cpu = CPU::new(BytesMut::new());


        let opCode = 0o07_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 210_u32;
        let og_rs_value = cpu.get_register_value(rs as u8);
        let instruction = formInstruction(opCode, rs, rd, immediate);
        cpu.execute(instruction);
        let rd_value = cpu.get_register_value(rd as u8);
        assert_eq!(i32::from_ne_bytes(rd_value.to_ne_bytes()), i32::from_ne_bytes(og_rs_value.to_ne_bytes()) * i32::from_ne_bytes(immediate.to_ne_bytes()))
    }
    #[test]
    fn MUL_IMMEDIATE_UNSIGNED() {
        let mut cpu = CPU::new(BytesMut::new());


        let opCode = 0o10_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 210_u32;
        let og_rs_value = cpu.get_register_value(rs as u8);
        let instruction = formInstruction(opCode, rs, rd, immediate);
        cpu.execute(instruction);
        let rd_value = cpu.get_register_value(rd as u8);
        assert_eq!(rd_value, og_rs_value * immediate)
    }

    #[test]
    fn DIV_IMMEDIATE() {
        let mut cpu = CPU::new(BytesMut::new());


        let opCode = 0o11_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 210_u32;
        let og_rs_value = cpu.get_register_value(rs as u8);
        let instruction = formInstruction(opCode, rs, rd, immediate);
        cpu.execute(instruction);
        let rd_value = cpu.get_register_value(rd as u8);
        assert_eq!(i32::from_ne_bytes(rd_value.to_ne_bytes()), i32::from_ne_bytes(og_rs_value.to_ne_bytes()) / i32::from_ne_bytes(immediate.to_ne_bytes()))
    }
    #[test]
    fn DIV_IMMEDIATE_UNSIGNED() {
        let mut cpu = CPU::new(BytesMut::new());


        let opCode = 0o12_u32;
        let rs = 5_u32;
        let rd = 10_u32;
        let immediate = 210_u32;
        let og_rs_value = cpu.get_register_value(rs as u8);
        let instruction = formInstruction(opCode, rs, rd, immediate);
        cpu.execute(instruction);
        let rd_value = cpu.get_register_value(rd as u8);
        assert_eq!(rd_value, og_rs_value / immediate)
    }

    fn formInstruction(opCode: u32, rs: u32, rd: u32, immediate: u32) -> u32 {
        return (opCode << 26) + (rs << 21) + (rd << 16) + immediate;
    }
    

}

