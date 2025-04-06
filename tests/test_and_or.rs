use mos_6502_emulator::{Cpu, Memory};

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> (Cpu, Memory) {
        let mut cpu = Cpu::new();
        let memory = Memory::new();
        cpu.reset();

        (cpu, memory)
    }

    #[test]
    fn test_and_immediate() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b1010_1111;
        mem.data[0xFFFC] = Cpu::AND_IMMEDIATE;
        mem.data[0xFFFD] = 0b1111_1010;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.a, 0b1010_1010);
    }

    #[test]
    fn test_and_zero_page() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b1010_1111;
        mem.data[0xFFFC] = Cpu::AND_ZERO_PAGE;
        mem.data[0xFFFD] = 0x43;
        mem.data[0x0043] = 0b1111_1010;
        cpu.execute(3, &mut mem);

        assert_eq!(cpu.a, 0b1010_1010);
    }

    #[test]
    fn test_and_zero_page_x() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b1010_1111;
        cpu.x = 0x1F;
        mem.data[0xFFFC] = Cpu::AND_ZERO_PAGE_X;
        mem.data[0xFFFD] = 0x43;
        mem.data[0x0062] = 0b1111_1010;
        cpu.execute(4, &mut mem);

        assert_eq!(cpu.a, 0b1010_1010);
    }

    #[test]
    fn test_and_absolute() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b1010_1111;
        mem.data[0xFFFC] = Cpu::AND_ABSOLUTE;
        mem.data[0xFFFD] = 0x44;
        mem.data[0xFFFE] = 0x33;
        mem.data[0x3344] = 0b1111_1010;
        cpu.execute(4, &mut mem);

        assert_eq!(cpu.a, 0b1010_1010);
    }

    #[test]
    fn test_and_absolute_x() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b1010_1111;
        cpu.x = 0x1F;
        mem.data[0xFFFC] = Cpu::AND_ABSOLUTE_X;
        mem.data[0xFFFD] = 0x44;
        mem.data[0xFFFE] = 0x33;
        mem.data[0x3363] = 0b1111_1010;
        cpu.execute(4, &mut mem);

        assert_eq!(cpu.a, 0b1010_1010);
    }

    #[test]
    fn test_and_absolute_y() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b1010_1111;
        cpu.y = 0x1F;
        mem.data[0xFFFC] = Cpu::AND_ABSOLUTE_Y;
        mem.data[0xFFFD] = 0x44;
        mem.data[0xFFFE] = 0x33;
        mem.data[0x3363] = 0b1111_1010;
        cpu.execute(4, &mut mem);

        assert_eq!(cpu.a, 0b1010_1010);
    }

    #[test]
    fn test_and_indirect_x() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b1010_1111;
        cpu.x = 0x1F;
        mem.data[0xFFFC] = Cpu::AND_INDIRECT_X;
        mem.data[0xFFFD] = 0x44;
        mem.data[0x0063] = 0x30;
        mem.data[0x0064] = 0x40;
        mem.data[0x4030] = 0b1111_1010;
        cpu.execute(6, &mut mem);

        assert_eq!(cpu.a, 0b1010_1010);
    }

    #[test]
    fn test_and_indirect_y() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b1010_1111;
        cpu.y = 0x1F;
        mem.data[0xFFFC] = Cpu::AND_INDIRECT_Y;
        mem.data[0xFFFD] = 0x44;
        mem.data[0x0044] = 0x30;
        mem.data[0x0045] = 0x40;
        mem.data[0x404F] = 0b1010_1010;
        cpu.execute(5, &mut mem);

        assert_eq!(cpu.a, 0b1010_1010);

        cpu.reset();

        cpu.a = 0b1010_1111;
        cpu.y = 0xFF;
        mem.data[0xFFFC] = Cpu::AND_INDIRECT_Y;
        mem.data[0xFFFD] = 0x44;
        mem.data[0x0044] = 0x30;
        mem.data[0x0045] = 0x40;
        mem.data[0x412F] = 0b1010_1010;
        cpu.execute(6, &mut mem);

        assert_eq!(cpu.a, 0b1010_1010);
    }

    #[test]
    fn test_eor_immediate() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b0000_1111;
        mem.data[0xFFFC] = Cpu::EOR_IMMEDIATE;
        mem.data[0xFFFD] = 0b1100_1100;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.a, 0b1100_1111);
    }

    #[test]
    fn test_eor_zero_page() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b0000_1111;
        mem.data[0xFFFC] = Cpu::EOR_ZERO_PAGE;
        mem.data[0xFFFD] = 0x45;
        mem.data[0x0045] = 0b1100_1100;
        cpu.execute(3, &mut mem);

        assert_eq!(cpu.a, 0b1100_1111);
    }

    #[test]
    fn test_eor_zero_page_x() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b0000_1111;
        cpu.x = 0x1F;
        mem.data[0xFFFC] = Cpu::EOR_ZERO_PAGE_X;
        mem.data[0xFFFD] = 0x45;
        mem.data[0x0064] = 0b1100_1100;
        cpu.execute(4, &mut mem);

        assert_eq!(cpu.a, 0b1100_1111);
    }

    #[test]
    fn test_eor_absolute() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b0000_1111;
        mem.data[0xFFFC] = Cpu::EOR_ABSOLUTE;
        mem.data[0xFFFD] = 0x40;
        mem.data[0xFFFE] = 0x30;
        mem.data[0x3040] = 0b1100_1100;
        cpu.execute(4, &mut mem);

        assert_eq!(cpu.a, 0b1100_1111);
    }

    #[test]
    fn test_eor_absolute_x() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b0000_1111;
        cpu.x = 0x1F;
        mem.data[0xFFFC] = Cpu::EOR_ABSOLUTE_X;
        mem.data[0xFFFD] = 0x40;
        mem.data[0xFFFE] = 0x30;
        mem.data[0x305F] = 0b1100_1100;
        cpu.execute(4, &mut mem);

        assert_eq!(cpu.a, 0b1100_1111);

        cpu.reset();
        cpu.a = 0b0000_1111;
        cpu.x = 0xFF;
        mem.data[0xFFFC] = Cpu::EOR_ABSOLUTE_X;
        mem.data[0xFFFD] = 0x40;
        mem.data[0xFFFE] = 0x30;
        mem.data[0x313F] = 0b1100_1100;
        cpu.execute(5, &mut mem);

        assert_eq!(cpu.a, 0b1100_1111);
    }

    #[test]
    fn test_eor_absolute_y() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b0000_1111;
        cpu.y = 0x1F;
        mem.data[0xFFFC] = Cpu::EOR_ABSOLUTE_Y;
        mem.data[0xFFFD] = 0x40;
        mem.data[0xFFFE] = 0x30;
        mem.data[0x305F] = 0b1100_1100;
        cpu.execute(4, &mut mem);

        assert_eq!(cpu.a, 0b1100_1111);

        cpu.reset();
        cpu.a = 0b0000_1111;
        cpu.y = 0xFF;
        mem.data[0xFFFC] = Cpu::EOR_ABSOLUTE_Y;
        mem.data[0xFFFD] = 0x40;
        mem.data[0xFFFE] = 0x30;
        mem.data[0x313F] = 0b1100_1100;
        cpu.execute(5, &mut mem);

        assert_eq!(cpu.a, 0b1100_1111);
    }

    #[test]
    fn test_eor_indirect_x() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b0000_1111;
        cpu.x = 0x1F;
        mem.data[0xFFFC] = Cpu::EOR_INDIRECT_X;
        mem.data[0xFFFD] = 0x44;
        mem.data[0x0063] = 0x30;
        mem.data[0x0064] = 0x40;
        mem.data[0x4030] = 0b1100_1100;
        cpu.execute(6, &mut mem);

        assert_eq!(cpu.a, 0b1100_1111);
    }

    #[test]
    fn test_eor_indirect_y() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b0000_1111;
        cpu.y = 0x1F;
        mem.data[0xFFFC] = Cpu::EOR_INDIRECT_Y;
        mem.data[0xFFFD] = 0x44;
        mem.data[0x0044] = 0x30;
        mem.data[0x0045] = 0x40;
        mem.data[0x404F] = 0b1100_1111;
        cpu.execute(5, &mut mem);

        assert_eq!(cpu.a, 0b1100_1111);

        cpu.reset();

        cpu.a = 0b0000_1111;
        cpu.y = 0xFF;
        mem.data[0xFFFC] = Cpu::EOR_INDIRECT_Y;
        mem.data[0xFFFD] = 0x44;
        mem.data[0x0044] = 0x30;
        mem.data[0x0045] = 0x40;
        mem.data[0x412F] = 0b1100_1100;
        cpu.execute(6, &mut mem);

        assert_eq!(cpu.a, 0b1100_1111);
    }
}
