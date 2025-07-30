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
    fn test_pha() {
        let (mut cpu, mut mem) = setup();

        cpu.a = 0x23;
        mem.data[0xFFFC] = Cpu::PHA_IMPLIED;

        cpu.execute(3, &mut mem);

        assert_eq!(cpu.a, mem.data[0x0100 + 0xFD]);
    }

    #[test]
    fn test_php() {
        let (mut cpu, mut mem) = setup();

        cpu.n = 1;
        cpu.v = 1;
        cpu.b = 1;
        cpu.d = 1;
        cpu.i = 1;
        cpu.z = 1;
        cpu.c = 1;

        mem.data[0xFFFC] = Cpu::PHP_IMPLIED;

        cpu.execute(3, &mut mem);

        assert_eq!(0b1111_1111, mem.data[0x0100 + 0xFD]);
    }

    #[test]
    fn test_pla() {
        let (mut cpu, mut mem) = setup();

        mem.data[0x0100 + 0xFE] = 0b1000_1000;
        mem.data[0xFFFC] = Cpu::PLA_IMPLIED;

        cpu.execute(4, &mut mem);

        assert_eq!(cpu.a, 0b1000_1000);
        assert_eq!(cpu.z, 0);
        assert_eq!(cpu.n, 1);
    }

    #[test]
    fn test_plp() {
        let (mut cpu, mut mem) = setup();

        mem.data[0x0100 + 0xFE] = 0b1111_1111;
        mem.data[0xFFFC] = Cpu::PLP_IMPLIED;

        cpu.execute(4, &mut mem);

        assert_eq!(cpu.n, 1);
        assert_eq!(cpu.v, 1);
        assert_eq!(cpu.b, 1);
        assert_eq!(cpu.d, 1);
        assert_eq!(cpu.i, 1);
        assert_eq!(cpu.z, 1);
        assert_eq!(cpu.c, 1);
    }
}