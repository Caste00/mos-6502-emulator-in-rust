use mos_6502_emulator::{Cpu, Memory};

fn main() {
    
    /* Debug */
    let mut mem = Memory::new();
    let mut cpu = Cpu::new();
    
    cpu.reset();

    mem.data[0xFFFC] = Cpu::JSR_ABSOLUTE;
    mem.data[0xFFFD] = 0x33;
    mem.data[0xFFFE] = 0x44;
    mem.data[0x4433] = Cpu::RST_IMPLIED;
    cpu.execute(12, &mut mem);

    println!("{:?}", cpu);
}
