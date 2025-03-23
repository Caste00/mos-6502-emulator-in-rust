use mos_6502_emulator::{Cpu, Memory};

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> (Cpu, Memory) {
        let mut cpu = Cpu::new(); 
        let mem = Memory::new();
        cpu.reset();

        (cpu, mem)
    }

    #[test]
    fn jsr_absolute() {
        let (mut cpu, mut mem) = setup();

        mem.data[0xFFFC] = Cpu::JSR_ABSOLUTE;
        mem.data[0xFFFD] = 0x33;
        mem.data[0xFFFE] = 0x44;
        mem.data[0x4433] = Cpu::LDA_IMMEDIATE;
        mem.data[0x4434] = 0x45;
        cpu.execute(8, &mut mem);

        assert_eq!(cpu.a, 0x45);
    }

    #[test]
    fn jmp_absolute() {
        let (mut cpu, mut mem) = setup();

        mem.data[0xFFFC] = Cpu::JMP_ABSOLUTE;
        mem.data[0xFFFD] = 0x44;
        mem.data[0xFFFE] = 0x33;
        mem.data[0x3344] = 0x45;
        cpu.execute(3, &mut mem);

        assert_eq!(cpu.pc, 0x45);
    }
}