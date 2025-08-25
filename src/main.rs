use mos_6502_emulator::{Cpu, Memory};

fn main() {
    
    /* Debug */
    let mut mem = Memory::new();
    let mut cpu = Cpu::new();
    
    cpu.reset();
    cpu.a = 0b1000_0001;
    cpu.c = 1;

    mem.data[0xFFFC] = Cpu::ROL_ACCUMULATOR;
    cpu.execute(2, &mut mem);

    println!("{}", cpu.a);
}
