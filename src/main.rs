use mos_6502_emulator::{Cpu, Memory};

fn main() {
    
    /* Debug */
    let mut mem = Memory::new();
    let mut cpu = Cpu::new();
    
    cpu.reset();

    cpu.y = 0xFF;
    mem.data[0xFFFC] = Cpu::LDA_INDIRECT_Y;
    mem.data[0xFFFD] = 0x32;
    mem.data[0x0032] = 0xA5;
    mem.data[0x0033] = 0x99;
    mem.data[0x99A6] = 0x45;

    cpu.execute(6, &mut mem);

    println!("{:?}", cpu);
}
