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

    #[test]
    fn test_bcs() {
        let (mut cpu, mut mem) = setup();

        cpu.c = 1;
        mem.data[0xFFFC] = Cpu::BCS_RELATIVE;
        mem.data[0xFFFD] = 0xF6;
        cpu.execute(3, &mut mem);

        assert_eq!(cpu.pc, 0xFFF4);

        cpu.reset();
        cpu.c = 0;
        mem.data[0xFFFC] = Cpu::BCS_RELATIVE;
        mem.data[0xFFFD] = 0xF6;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.pc, 0xFFFE);
    }

    #[test]
    fn test_beq() {
        let (mut cpu, mut mem) = setup();

        cpu.z = 1;
        mem.data[0xFFFC] = Cpu::BEQ_RELATIVE;
        mem.data[0xFFFD] = 0xF6;
        cpu.execute(3, &mut mem);

        assert_eq!(cpu.pc, 0xFFF4);

        cpu.reset();
        cpu.z = 0;
        mem.data[0xFFFC] = Cpu::BEQ_RELATIVE;
        mem.data[0xFFFD] = 0xF6;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.pc, 0xFFFE);
    }

    #[test]
    fn test_bne() {
        let (mut cpu, mut mem) = setup();

        cpu.z = 0;
        mem.data[0xFFFC] = Cpu::BNE_RELATIVE;
        mem.data[0xFFFD] = 0xF6;
        cpu.execute(3, &mut mem);

        assert_eq!(cpu.pc, 0xFFF4);

        cpu.reset();
        cpu.z = 1;
        mem.data[0xFFFC] = Cpu::BNE_RELATIVE;
        mem.data[0xFFFD] = 0xF6;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.pc, 0xFFFE);
    }

    #[test]
    fn test_bmi() {
        let (mut cpu, mut mem) = setup();

        cpu.n = 1;
        mem.data[0xFFFC] = Cpu::BMI_RELATIVE;
        mem.data[0xFFFD] = 0xF6;
        cpu.execute(3, &mut mem);

        assert_eq!(cpu.pc, 0xFFF4);

        cpu.reset();
        cpu.n = 0;
        mem.data[0xFFFC] = Cpu::BMI_RELATIVE;
        mem.data[0xFFFD] = 0xF6;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.pc, 0xFFFE);
    }

    #[test]
    fn test_bpl() {
        let (mut cpu, mut mem) = setup();

        cpu.n = 0;
        mem.data[0xFFFC] = Cpu::BPL_RELATIVE;
        mem.data[0xFFFD] = 0xF6;
        cpu.execute(3, &mut mem);

        assert_eq!(cpu.pc, 0xFFF4);

        cpu.reset();
        cpu.n = 1;
        mem.data[0xFFFC] = Cpu::BPL_RELATIVE;
        mem.data[0xFFFD] = 0xF6;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.pc, 0xFFFE);
    }

    #[test]
    fn test_bvs() {
        let (mut cpu, mut mem) = setup();

        cpu.v = 1;
        mem.data[0xFFFC] = Cpu::BVS_RELATIVE;
        mem.data[0xFFFD] = 0xF6;
        cpu.execute(3, &mut mem);

        assert_eq!(cpu.pc, 0xFFF4);

        cpu.reset();
        cpu.v = 0;
        mem.data[0xFFFC] = Cpu::BVS_RELATIVE;
        mem.data[0xFFFD] = 0xF6;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.pc, 0xFFFE);
    }

    #[test]
    fn test_bvc() {
        let (mut cpu, mut mem) = setup();

        cpu.v = 0;
        mem.data[0xFFFC] = Cpu::BVC_RELATIVE;
        mem.data[0xFFFD] = 0xF6;
        cpu.execute(3, &mut mem);

        assert_eq!(cpu.pc, 0xFFF4);

        cpu.reset();
        cpu.v = 1;
        mem.data[0xFFFC] = Cpu::BVC_RELATIVE;
        mem.data[0xFFFD] = 0xF6;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.pc, 0xFFFE);
    }
}