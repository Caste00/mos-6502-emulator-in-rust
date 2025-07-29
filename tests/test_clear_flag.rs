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
    fn test_clc() {
        let (mut cpu, mut mem) = setup();

        cpu.c = 1;
        assert_eq!(cpu.c, 1);

        mem.data[0xFFFC] = Cpu::CLC_IMPLIED;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.c, 0);
    }
    #[test]
    fn test_cld() {
        let (mut cpu, mut mem) = setup();

        cpu.d = 1;
        assert_eq!(cpu.d, 1);

        mem.data[0xFFFC] = Cpu::CLD_IMPLIED;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.d, 0);
    }

    #[test]
    fn test_cli() {
        let (mut cpu, mut mem) = setup();

        cpu.i = 1;
        assert_eq!(cpu.i, 1);

        mem.data[0xFFFC] = Cpu::CLI_IMPLIED;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.i, 0);
    }

    #[test]
    fn test_clv() {
        let (mut cpu, mut mem) = setup();

        cpu.v = 1;
        assert_eq!(cpu.v, 1);

        mem.data[0xFFFC] = Cpu::CLV_IMPLIED;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.v, 0);
    }

    #[test]
    fn test_sec() {
        let (mut cpu, mut mem) = setup();
        
        cpu.c = 0;
        assert_eq!(cpu.c, 0);

        mem.data[0xFFFC] = Cpu::SEC_IMPLIED;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.c, 1);
    }

    #[test]
    fn test_sed() {
        let (mut cpu, mut mem) = setup();
        
        cpu.d = 0;
        assert_eq!(cpu.d, 0);

        mem.data[0xFFFC] = Cpu::SED_IMPLIED;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.d, 1);
    }

    #[test]
    fn test_sei() {
        let (mut cpu, mut mem) = setup();

        cpu.i = 0;
        assert_eq!(cpu.i, 0);

        mem.data[0xFFFC] = Cpu::SEI_IMPLIED;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.i, 1);
    }
}