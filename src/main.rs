mod cpu;
mod memory;

fn main() {
    let mut cpu: cpu::CPU = cpu::CPU::new(memory::Memory::new(1000));
    cpu.execute(0)
}
