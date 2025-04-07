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
    fn test_bcc() {
        let (mut cpu, mut mem) = setup();

        cpu.c = 0;
        mem.data[0xFFFC] = Cpu::BBC_RELATIVE;
        mem.data[0xFFFD] = 0xF6;
        cpu.execute(3, &mut mem);
        
        assert_eq!(cpu.pc, 0xFFF4);

        cpu.reset();
        cpu.c = 1;
        mem.data[0xFFFC] = Cpu::BBC_RELATIVE;
        mem.data[0xFFFD] = 0xF6;
        cpu.execute(2, &mut mem);
        
        assert_eq!(cpu.pc, 0xFFFE);
    }
}