use mos_6502_emulator::{Cpu, Memory};

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (Cpu, Memory) {
        let mut cpu = Cpu::new(); 
        let mem = Memory::new();
        cpu.reset();

        (cpu, mem)
    }
    
    #[test]
    fn test_lda_immediate() {
        let (mut cpu, mut mem) = setup();

        mem.data[0xFFFC] = Cpu::LDA_IMMEDIATE;
        mem.data[0xFFFD] = 0x45;
        cpu.execute(2, &mut mem);
        
        assert_eq!(cpu.a, 0x45);
    }

    #[test]
    fn test_lda_zero_page() {
        let (mut cpu, mut mem) = setup();

        mem.data[0xFFFC] = Cpu::LDA_ZERO_PAGE;
        mem.data[0xFFFD] = 0x32;
        mem.data[0x0032] = 0x45;
        cpu.execute(3, &mut mem);

        assert_eq!(cpu.a, 0x45);
    }

    #[test]
    fn test_lda_zero_page_x() {
        let (mut cpu, mut mem) = setup();

        cpu.x = 0x10;
        mem.data[0xFFFC] = Cpu::LDA_ZERO_PAGE_X;
        mem.data[0xFFFD] = 0x32;
        mem.data[0x42] = 0x45;
        cpu.execute(4, &mut mem);

        assert_eq!(cpu.a, 0x45);
    }

    #[test]
    fn test_lda_absolute() {
        let (mut cpu, mut mem) = setup();

        mem.data[0xFFFC] = Cpu::LDA_ABSOLUTE;
        mem.data[0xFFFD] = 0x12;
        mem.data[0xFFFE] = 0x34;
        mem.data[0x3412] = 0x45;
        cpu.execute(5, &mut mem);

        assert_eq!(cpu.a, 0x45);
    }

    #[test]
    fn test_lda_absolute_x() {
        let (mut cpu, mut mem) = setup();

        cpu.x = 0x12;
        mem.data[0xFFFC] = Cpu::LDA_ABSOLUTE_X;
        mem.data[0xFFFD] = 0x33;
        mem.data[0xFFFE] = 0x44;
        mem.data[0x4445] = 0x45;
        cpu.execute(4, &mut mem);

        assert_eq!(cpu.a, 0x45);

        cpu.reset();
        mem.reset();

        cpu.x = 0xFF;
        mem.data[0xFFFC] = Cpu::LDA_ABSOLUTE_X;
        mem.data[0xFFFD] = 0x33;
        mem.data[0xFFFE] = 0x44;
        mem.data[0x4532] =0x45;
        cpu.execute(5, &mut mem);

        assert_eq!(cpu.a, 0x45);
    }

    #[test]
    fn test_lda_absolute_y() {
        let (mut cpu, mut mem) = setup();

        cpu.y = 0x12;
        mem.data[0xFFFC] = Cpu::LDA_ABSOLUTE_Y;
        mem.data[0xFFFD] = 0x33;
        mem.data[0xFFFE] = 0x44;
        mem.data[0x4445] = 0x45;
        cpu.execute(4, &mut mem);

        assert_eq!(cpu.a, 0x45);

        cpu.reset();
        mem.reset();

        cpu.y = 0xFF;
        mem.data[0xFFFC] = Cpu::LDA_ABSOLUTE_Y;
        mem.data[0xFFFD] = 0x33;
        mem.data[0xFFFE] = 0x44;
        mem.data[0x4532] =0x45;
        cpu.execute(5, &mut mem);

        assert_eq!(cpu.a, 0x45);
    }

    #[test]
    fn lda_indirect_x() {
        let (mut cpu, mut mem) = setup();

        cpu.x = 0x01;
        mem.data[0xFFFC] = Cpu::LDA_INDIRECT_X;
        mem.data[0xFFFD] = 0x20;
        mem.data[0x21] = 0x33;
        mem.data[0x22] = 0x44;
        mem.data[0x4433] = 0x45;
        cpu.execute(6, &mut mem);

        assert_eq!(cpu.a, 0x45);
    }
}