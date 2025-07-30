use mos_6502_emulator::{Cpu, Memory};

fn main() {
    
    /* Debug */
    let mut mem = Memory::new();
    let mut cpu = Cpu::new();
    
    cpu.reset();
    cpu.sp = 0xFD;
    mem.data[0x0100 + 0xFE] = 0b1000_1000;
    mem.data[0xFFFC] = Cpu::PLA_IMPLIED;

    cpu.execute(4, &mut mem);

    println!("{}", cpu.a);
}
