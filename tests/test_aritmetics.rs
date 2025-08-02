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
    fn test_adc_immediate() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b1000_0000;
        mem.data[0xFFFC] = Cpu::ADC_IMMEDIATE;
        mem.data[0xFFFD] = 0b1000_0000;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.c, 1);
        assert_eq!(cpu.z, 1);
        assert_eq!(cpu.v, 1);
        assert_eq!(cpu.n, 0);

        cpu.a = 0b0000_0000;
        cpu.c = 0;
        cpu.pc = 0xFFFC;
        mem.data[0xFFFC] = Cpu::ADC_IMMEDIATE;
        mem.data[0xFFFD] = 0b1000_0000;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.a, 0b1000_0000);
        assert_eq!(cpu.c, 0);
        assert_eq!(cpu.z, 0);
        assert_eq!(cpu.v, 0);
        assert_eq!(cpu.n, 1);
    }

    #[test]
    fn test_adc_zero_page() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b0101_0101;
        mem.data[0xFFFC] = Cpu::ADC_ZERO_PAGE;
        mem.data[0xFFFD] = 0xAB;
        mem.data[0xAB] = 0b0000_1111;
        cpu.execute(3, &mut mem);

        assert_eq!(cpu.a, 0b1100100);
    }

    #[test]
    fn test_adc_zero_page_x() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b0101_0101;
        cpu.x = 0x10;
        mem.data[0xFFFC] = Cpu::ADC_ZERO_PAGE_X;
        mem.data[0xFFFD] = 0xAB;
        mem.data[0xAB + 0x10] = 0b0000_1111;
        cpu.execute(4, &mut mem);

        assert_eq!(cpu.a, 0b1100100);
    }

    #[test]
    fn test_adc_absolute() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b0101_0101;
        mem.data[0xFFFC] = Cpu::ADC_ABSOLUTE;
        mem.data[0xFFFD] = 0xAB;
        mem.data[0xFFFE] = 0xAB;
        mem.data[0xABAB] = 0b0000_1111;
        cpu.execute(4, &mut mem);

        assert_eq!(cpu.a, 0b1100100);
    }

    #[test]
    fn test_adc_absolute_x() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b0101_0101;
        cpu.x = 0x50;
        mem.data[0xFFFC] = Cpu::ADC_ABSOLUTE_X;
        mem.data[0xFFFD] = 0xF0;
        mem.data[0xFFFE] = 0xAB;
        mem.data[0xABF0 + 0x50] = 0b0000_1111;
        cpu.execute(5, &mut mem);

        assert_eq!(cpu.a, 0b1100100);
    }

    #[test]
    fn test_adc_absolute_y() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b0101_0101;
        cpu.y = 0x05;
        mem.data[0xFFFC] = Cpu::ADC_ABSOLUTE_Y;
        mem.data[0xFFFD] = 0x00;
        mem.data[0xFFFE] = 0x20;
        mem.data[0x2000 + 0x05] = 0b0000_1111;
        cpu.execute(4, &mut mem);

        assert_eq!(cpu.a, 0b1100100);
    }

    #[test]
    fn test_adc_indirect_x() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b0101_0101;
        cpu.x = 0x10;
        mem.data[0xFFFC] = Cpu::ADC_INDIRECT_X;
        mem.data[0xFFFD] = 0xFF;
        mem.data[0x000F] = 0xAB;
        mem.data[0x0010] = 0xCD;
        mem.data[0xCDAB] = 0b0000_1111;
        cpu.execute(6, &mut mem);

        assert_eq!(cpu.a, 0b1100100);
    }

    #[test]
    fn test_adc_indirect_y() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0x01;
        cpu.y = 0x01;
        mem.data[0xFFFC] = Cpu::ADC_INDIRECT_Y;
        mem.data[0xFFFD] = 0x10;
        mem.data[0x0010] = 0xFF;
        mem.data[0x0011] = 0x01;
        mem.data[0x0200] = 0x0F;
        cpu.execute(6, &mut mem);
        
        assert_eq!(cpu.a, 0x10);
    }

    #[test]
    fn test_sbc_immediate() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0b1000_0000;
        cpu.c = 1;
        mem.data[0xFFFC] = Cpu::SBC_IMMEDIATE;
        mem.data[0xFFFD] = 0b1000_0000;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.c, 1);
        assert_eq!(cpu.z, 1);
        assert_eq!(cpu.v, 1);
        assert_eq!(cpu.n, 0);

        cpu.a = 0b0000_0000;
        cpu.c = 0;
        cpu.pc = 0xFFFC;
        mem.data[0xFFFC] = Cpu::SBC_IMMEDIATE;
        mem.data[0xFFFD] = 0b1000_0000;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.a, 0b0111_1111);
        assert_eq!(cpu.c, 0);
        assert_eq!(cpu.z, 0);
        assert_eq!(cpu.v, 0);
        assert_eq!(cpu.n, 0);
    }
}