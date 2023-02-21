use num_derive::FromPrimitive;

pub struct FPU {
    registers: [f32; 32]
}

impl FPU {
    fn execute(&mut self, instruction: u32) {
        let cop1_selector = (instruction & 0b11111_00000_00000_00000_00000_000000) >> 21;
        let selector: COP1 = num::FromPrimitive::from_u32(instruction).unwrap();
        match selector {
            COP1::FMTS => todo!(),
            COP1::FMTW => todo!(),
            COP1::BC => todo!(),
            COP1::MF => todo!(),
            COP1::MT => todo!(),
            COP1::CF => todo!(),
            COP1::CT => todo!(),
        }
    }

}

#[derive(FromPrimitive)]
pub enum COP1 {
    FMTS = 16,
    FMTW = 20,
    BC = 0b01000,
    MF = 0b00000,
    MT = 0b00100,
    CF = 0b00010,
    CT = 0b00110
}

pub enum FmtOperation {
    ABS = 0o05,
    CEIL_L = 0o12,
    CEIL_W = 0o16,
    CVT_D = 0o41,
    CVT_S = 0o40,
    DIV = 0o03,
    MOV = 0o06,
    MOVCF = 0o21,
    MOVN = 0o23,
    MUL = 0o02,
    NEG = 0o07,
    SUM = 0o01
}


