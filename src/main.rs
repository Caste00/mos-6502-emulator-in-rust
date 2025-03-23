use mos_6502_emulator::{Cpu, Memory};

fn main() {
    let mut mem = Memory::new();
    let mut cpu = Cpu::new();
    
    /* Debug */
    cpu.reset();

    cpu.x = 0x12;
    //cpu.x = 0xFF;

    mem.data[0xFFFC] = Cpu::LDA_ABSOLUTE_X;
    mem.data[0xFFFD] = 0x33;
    mem.data[0xFFFE] = 0x44;
    mem.data[0x4433] = 23;
    mem.data[0x4534] = 32;

    cpu.execute(5, &mut mem);

    println!("{:?}", cpu);

}
