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

    /* Test STA */
    #[test]
    fn test_sta_zero_page() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0x45;
        mem.data[0xFFFC] = Cpu::STA_ZERO_PAGE;
        mem.data[0xFFFD] = 0x43;
        cpu.execute(3, &mut mem);

        assert_eq!(mem.data[0x0043], 0x45);
    }

    #[test]
    fn test_sta_zero_page_x() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0x45;
        cpu.x = 0x03;
        mem.data[0xFFFC] = Cpu::STA_ZERO_PAGE_X;
        mem.data[0xFFFD] = 0x40;
        cpu.execute(4, &mut mem);

        assert_eq!(mem.data[0x0043], 0x45);
    }

    #[test]
    fn test_sta_absolute() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0x45;
        mem.data[0xFFFC] = Cpu::STA_ABSOLUTE;
        mem.data[0xFFFD] = 0x43;
        mem.data[0xFFFE] = 0x22;
        cpu.execute(4, &mut mem);

        assert_eq!(mem.data[0x2243], 0x45);
    }

    #[test]
    fn test_sta_absolute_x() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0x45;
        cpu.x = 0x05;
        mem.data[0xFFFC] = Cpu::STA_ABSOLUTE_X;
        mem.data[0xFFFD] = 0x43;
        mem.data[0xFFFE] = 0x22;
        cpu.execute(5, &mut mem);

        assert_eq!(mem.data[0x2248], 0x45);
    }

    #[test]
    fn test_sta_absolute_y() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0x45;
        cpu.y = 0x05;
        mem.data[0xFFFC] = Cpu::STA_ABSOLUTE_Y;
        mem.data[0xFFFD] = 0x43;
        mem.data[0xFFFE] = 0x22;
        cpu.execute(5, &mut mem);

        assert_eq!(mem.data[0x2248], 0x45);
    }

    #[test]
    fn test_sta_indirect_x() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0x45;
        cpu.x = 0x05;
        mem.data[0xFFFC] = Cpu::STA_INDIRECT_X;
        mem.data[0xFFFD] = 0x43;
        mem.data[0x0048] = 0x10;
        mem.data[0x0049] = 0x20;
        cpu.execute(6, &mut mem);

        assert_eq!(mem.data[0x2010], 0x45);
    }

    #[test]
    fn test_sta_indirect_y() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0x45;
        cpu.y = 0x05;
        mem.data[0xFFFC] = Cpu::STA_INDIRECT_Y;
        mem.data[0xFFFD] = 0x48;
        mem.data[0x0048] = 0x10;
        mem.data[0x0049] = 0x20;
        cpu.execute(6, &mut mem);

        assert_eq!(mem.data[0x2015], 0x45);
    }

    /* Test STX */
    #[test]
    fn test_stx_zero_page() {
        let (mut cpu, mut mem) = setup();

        cpu.x = 0x45;
        mem.data[0xFFFC] = Cpu::STX_ZERO_PAGE;
        mem.data[0xFFFD] = 0x43;
        cpu.execute(3, &mut mem);

        assert_eq!(mem.data[0x0043], 0x45);
    }

    #[test]
    fn test_stx_zero_page_y() {
        let (mut cpu, mut mem) = setup();

        cpu.x = 0x45;
        cpu.y = 0x03;
        mem.data[0xFFFC] = Cpu::STX_ZERO_PAGE_Y;
        mem.data[0xFFFD] = 0x40;
        cpu.execute(4, &mut mem);

        assert_eq!(mem.data[0x0043], 0x45);
    }

    #[test]
    fn test_stx_absolute() {
        let (mut cpu, mut mem) = setup();

        cpu.x = 0x45;
        mem.data[0xFFFC] = Cpu::STX_ABSOLUTE;
        mem.data[0xFFFD] = 0x43;
        mem.data[0xFFFE] = 0x22;
        cpu.execute(4, &mut mem);

        assert_eq!(mem.data[0x2243], 0x45);
    }

        /* Test STY */
        #[test]
        fn test_sty_zero_page() {
            let (mut cpu, mut mem) = setup();
    
            cpu.y = 0x45;
            mem.data[0xFFFC] = Cpu::STY_ZERO_PAGE;
            mem.data[0xFFFD] = 0x43;
            cpu.execute(3, &mut mem);
    
            assert_eq!(mem.data[0x0043], 0x45);
        }
    
        #[test]
        fn test_sty_zero_page_x() {
            let (mut cpu, mut mem) = setup();
    
            cpu.y = 0x45;
            cpu.x = 0x03;
            mem.data[0xFFFC] = Cpu::STY_ZERO_PAGE_X;
            mem.data[0xFFFD] = 0x40;
            cpu.execute(4, &mut mem);
    
            assert_eq!(mem.data[0x0043], 0x45);
        }
    
        #[test]
        fn test_sty_absolute() {
            let (mut cpu, mut mem) = setup();
    
            cpu.y = 0x45;
            mem.data[0xFFFC] = Cpu::STY_ABSOLUTE;
            mem.data[0xFFFD] = 0x43;
            mem.data[0xFFFE] = 0x22;
            cpu.execute(4, &mut mem);
    
            assert_eq!(mem.data[0x2243], 0x45);
        }
}