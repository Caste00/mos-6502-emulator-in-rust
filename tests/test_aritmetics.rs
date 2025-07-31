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
}