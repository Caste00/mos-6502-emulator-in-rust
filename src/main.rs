use mos_6502_emulator::{Cpu, Memory};

fn main() {
    
    /* Debug */
    let mut mem = Memory::new();
    let mut cpu = Cpu::new();
    
    cpu.reset();
    cpu.a = 0b1000_0000;
    mem.data[0xFFFC] = Cpu::ADC_IMMEDIATE;
    mem.data[0xFFFD] = 0b1000_0000;
    cpu.execute(3, &mut mem);

    println!("{}", cpu.a);
    println!("c: {}, z: {}, v: {}, n: {}", cpu.c, cpu.z, cpu.v, cpu.n);

    cpu.a = 0b0000_0000;
    cpu.pc = 0xFFFC;
    mem.data[0xFFFC] = Cpu::ADC_IMMEDIATE;
    mem.data[0xFFFD] = 0b1000_0000;
    cpu.execute(2, &mut mem);

    println!("{}", cpu.a);
    println!("c: {}, z: {}, v: {}, n: {}", cpu.c, cpu.z, cpu.v, cpu.n);
}
