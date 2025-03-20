mod cpu;
mod memory;

use memory::memory::Memory;
use cpu::cpu::Cpu;

fn main() {
    let mut mem = Memory::new();
    let mut cpu = Cpu::new();

    /* Debug */
    cpu.reset();

    mem.data[0xFFFC] = Cpu::JSR_ABSOLUTE;
    mem.data[0xFFFD] = 0x33;
    mem.data[0xFFFE] = 0x44;
    mem.data[0x4433] = Cpu::LDA_IMMEDIATE;
    mem.data[0x4434] = 0x43;
    cpu.execute(8, &mut mem);

    println!("{:#?}", cpu);

}
