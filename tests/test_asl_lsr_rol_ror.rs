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
    fn asl_accumulator() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b1010_1010;
        mem.data[0xFFFC] = Cpu::ASL_ACCUMULATOR;
        cpu.execute(2, &mut mem);
        
        assert_eq!(cpu.a, 0b0101_0100);
        assert_eq!(cpu.c, 1);
    }

    #[test]
    fn asl_zero_page() {
        let (mut cpu, mut mem) = setup();

        mem.data[0xFFFC] = Cpu::ASL_ZERO_PAGE;
        mem.data[0xFFFD] = 0x42;
        mem.data[0x0042] = 0b1010_1010;
        cpu.execute(5, &mut mem);
        
        assert_eq!(mem.data[0x0042], 0b0101_0100);
    }

    #[test]
    fn asl_zero_page_x() {
        let (mut cpu, mut mem) = setup();

        cpu.x = 0x01;
        mem.data[0xFFFC] = Cpu::ASL_ZERO_PAGE_X;
        mem.data[0xFFFD] = 0x42;
        mem.data[0x0043] = 0b1010_1010;
        cpu.execute(6, &mut mem);
        
        assert_eq!(mem.data[0x0043], 0b0101_0100);
    }

    #[test]
    fn asl_absolute() {
        let (mut cpu, mut mem) = setup();

        mem.data[0xFFFC] = Cpu::ASL_ABSOLUTE;
        mem.data[0xFFFD] = 0x40;
        mem.data[0xFFFE] = 0x33;
        mem.data[0x3340] = 0b1010_1010;
        cpu.execute(6, &mut mem);
        
        assert_eq!(mem.data[0x3340], 0b0101_0100);
    }

    #[test]
    fn asl_absolute_x() {
        let (mut cpu, mut mem) = setup();

        cpu.x = 0x01;
        mem.data[0xFFFC] = Cpu::ASL_ABSOLUTE_X;
        mem.data[0xFFFD] = 0x40;
        mem.data[0xFFFE] = 0x33;
        mem.data[0x3341] = 0b1010_1010;
        cpu.execute(7, &mut mem);
        
        assert_eq!(mem.data[0x3341], 0b0101_0100);
    }

    #[test]
    fn lsr_accumulator() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b1010_1010;
        mem.data[0xFFFC] = Cpu::LSR_ACCUMULATOR;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.a, 0b0101_0101);
    }

    #[test]
    fn lsr_zero_page() {
        let (mut cpu, mut mem) = setup();
        
        mem.data[0xFFFC] = Cpu::LSR_ZERO_PAGE;
        mem.data[0xFFFD] = 0x42;
        mem.data[0x0042] = 0b1010_1010;
        cpu.execute(5, &mut mem);

        assert_eq!(mem.data[0x0042], 0b0101_0101);
    }

    #[test]
    fn lsr_zero_page_x() {
        let (mut cpu, mut mem) = setup();

        cpu.x = 0x01;
        mem.data[0xFFFC] = Cpu::LSR_ZERO_PAGE_X;
        mem.data[0xFFFD] = 0x42;
        mem.data[0x0043] = 0b1010_1010;
        cpu.execute(6, &mut mem);

        assert_eq!(mem.data[0x0043], 0b0101_0101);
    }

    #[test]
    fn lsr_absolute() {
        let (mut cpu, mut mem) = setup();

        mem.data[0xFFFC] = Cpu::LSR_ABSOLUTE;
        mem.data[0xFFFD] = 0x40;
        mem.data[0xFFFE] = 0x33;
        mem.data[0x3340] = 0b1010_1010;
        cpu.execute(6, &mut mem);

        assert_eq!(mem.data[0x3340], 0b0101_0101);
    }

    #[test]
    fn lsr_absolute_x() {
        let (mut cpu, mut mem) = setup();

        cpu.x = 0x01;
        mem.data[0xFFFC] = Cpu::LSR_ABSOLUTE_X;
        mem.data[0xFFFD] = 0x40;
        mem.data[0xFFFE] = 0x33;
        mem.data[0x3341] = 0b1010_1010;
        cpu.execute(7, &mut mem);

        assert_eq!(mem.data[0x3341], 0b0101_0101);
    }

    #[test]
    fn rol_accumulator() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b0000_1000;
        cpu.c = 1;
        mem.data[0xFFFC] = Cpu::ROL_ACCUMULATOR;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.a, 0b0001_0001);
        assert_eq!(cpu.z, 0);
        assert_eq!(cpu.c, 0);
        assert_eq!(cpu.n, 0);
    }

    #[test]
    fn rol_zero_page() {
        let (mut cpu, mut mem) = setup();

        cpu.c = 1;
        mem.data[0xFFFC] = Cpu::ROL_ZERO_PAGE;
        mem.data[0xFFFD] = 0x42;
        mem.data[0x0042] = 0b0010_1010;
        cpu.execute(5, &mut mem);

        assert_eq!(mem.data[0x0042], 0b0101_0101);
        assert_eq!(cpu.c, 0);
    }

    #[test]
    fn rol_zero_page_x() {
        let (mut cpu, mut mem) = setup();

        cpu.x = 0x01;
        cpu.c = 0;
        mem.data[0xFFFC] = Cpu::ROL_ZERO_PAGE_X;
        mem.data[0xFFFD] = 0x42;
        mem.data[0x0043] = 0b1010_1011;
        cpu.execute(6, &mut mem);

        assert_eq!(mem.data[0x0043], 0b0101_0110);
        assert_eq!(cpu.c, 1);
    }

    #[test]
    fn rol_absolute() {
        let (mut cpu, mut mem) = setup();

        cpu.c = 0;
        mem.data[0xFFFC] = Cpu::ROL_ABSOLUTE;
        mem.data[0xFFFD] = 0x40;
        mem.data[0xFFFE] = 0x33;
        mem.data[0x3340] = 0b1010_1010;
        cpu.execute(6, &mut mem);

        assert_eq!(mem.data[0x3340], 0b0101_0100);
    }

    #[test]
    fn rol_absolute_x() {
        let (mut cpu, mut mem) = setup();

        cpu.c = 1;
        cpu.x = 0x01;
        mem.data[0xFFFC] = Cpu::ROL_ABSOLUTE_X;
        mem.data[0xFFFD] = 0x40;
        mem.data[0xFFFE] = 0x33;
        mem.data[0x3341] = 0b1010_1010;
        cpu.execute(7, &mut mem);

        assert_eq!(mem.data[0x3341], 0b0101_0101);
    }

    #[test]
    fn ror_accumulator() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b0000_1001;
        cpu.c = 1;
        mem.data[0xFFFC] = Cpu::ROR_ACCUMULATOR;
        cpu.execute(2, &mut mem);
        
        assert_eq!(cpu.a, 0b1000_0100);
        assert_eq!(cpu.z, 0);
        assert_eq!(cpu.c, 1);
        assert_eq!(cpu.n, 1);
    }

    #[test]
    fn ror_zero_page() {
        let (mut cpu, mut mem) = setup();

        cpu.c = 0;
        mem.data[0xFFFC] = Cpu::ROR_ZERO_PAGE;
        mem.data[0xFFFD] = 0x42;
        mem.data[0x0042] = 0b0010_1011;
        cpu.execute(5, &mut mem);

        assert_eq!(mem.data[0x0042], 0b0001_0101);
        assert_eq!(cpu.c, 1); // LSB originale = 1
    }

    #[test]
    fn ror_zero_page_x() {
        let (mut cpu, mut mem) = setup();

        cpu.x = 0x01;
        cpu.c = 1;
        mem.data[0xFFFC] = Cpu::ROR_ZERO_PAGE_X;
        mem.data[0xFFFD] = 0x42;
        mem.data[0x0043] = 0b1010_1010;
        cpu.execute(6, &mut mem);

        assert_eq!(mem.data[0x0043], 0b1101_0101);
        assert_eq!(cpu.c, 0);
    }

    #[test]
    fn ror_absolute() {
        let (mut cpu, mut mem) = setup();

        cpu.c = 0;
        mem.data[0xFFFC] = Cpu::ROR_ABSOLUTE;
        mem.data[0xFFFD] = 0x40;
        mem.data[0xFFFE] = 0x33;
        mem.data[0x3340] = 0b1010_1011;
        cpu.execute(6, &mut mem);

        assert_eq!(mem.data[0x3340], 0b0101_0101);
        assert_eq!(cpu.c, 1);
    }

    #[test]
    fn ror_absolute_x() {
        let (mut cpu, mut mem) = setup();

        cpu.c = 1;
        cpu.x = 0x01;
        mem.data[0xFFFC] = Cpu::ROR_ABSOLUTE_X;
        mem.data[0xFFFD] = 0x40;
        mem.data[0xFFFE] = 0x33;
        mem.data[0x3341] = 0b0010_1010;
        cpu.execute(7, &mut mem);

        assert_eq!(mem.data[0x3341], 0b1001_0101);
        assert_eq!(cpu.c, 0);
    }

}