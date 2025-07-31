use mos_6502_emulator::{Cpu, Memory};

fn main() {
    
    /* Debug */
    let mut mem = Memory::new();
    let mut cpu = Cpu::new();
    
    cpu.reset();
    cpu.a = 0b0101_0101;
    cpu.x = 0x50;
    mem.data[0xFFFC] = Cpu::ADC_ABSOLUTE_X;
    mem.data[0xFFFD] = 0xF0;
    mem.data[0xFFFE] = 0xAB;
    mem.data[0xABF0 + 0x50] = 0b0000_1111;
    cpu.execute(5, &mut mem);

    println!("{}, {}", cpu.a, mem.data[0xABF0 + 0x50]);
}
