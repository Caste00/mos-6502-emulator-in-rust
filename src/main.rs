use mos_6502_emulator::{Cpu, Memory};

fn main() {
    
    /* Debug */
    let mut mem = Memory::new();
    let mut cpu = Cpu::new();
    
    cpu.reset();

    cpu.a = 0b1010_1111;
    cpu.x = 0x1F;
    mem.data[0xFFFC] = Cpu::AND_INDIRECT_X;
    mem.data[0xFFFD] = 0x44;
    mem.data[0x0063] = 0x30;
    mem.data[0x0064] = 0x40;
    mem.data[0x4030] = 0b1111_1010;
    cpu.execute(5, &mut mem);

    println!("{:?}", cpu);
}
