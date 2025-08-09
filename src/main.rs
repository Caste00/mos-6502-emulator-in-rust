use mos_6502_emulator::{Cpu, Memory};

fn main() {
    
    /* Debug */
    let mut mem = Memory::new();
    let mut cpu = Cpu::new();
    
    cpu.reset();
    cpu.a = 48;
    mem.data[0xFFFC] = Cpu::CMP_IMMEDIATE;
    mem.data[0xFFFD] = 0x19;
    mem.data[0x0019] = 48;
    cpu.execute(2, &mut mem);

    println!("{}", mem.data[0x0019]);
}
