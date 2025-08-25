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
    fn test_brk_implied() {
        let (mut cpu, mut mem) = setup();

        mem.data[0xFFFC] = Cpu::BRK_IMPLIED;
        mem.data[0xFFFE] = 0x00;
        mem.data[0xFFFF] = 0x20;
        cpu.execute(7, &mut mem);

        assert_eq!(cpu.pc, 0x2000);
        let sp = cpu.sp as usize + 1;
        let status = mem.data[0x0100 + sp + 0];
        let pcl    = mem.data[0x0100 + sp + 1];
        let pch    = mem.data[0x0100 + sp + 2];

        assert_eq!(((pch as u16) << 8) | pcl as u16, 0xFFFE);
        assert_eq!(status & 0b0011_0000, 0b0011_0000);
        assert_eq!(cpu.i, 1);
    }

    #[test]
    fn test_nop_implied() {
        let (mut cpu, mut mem) = setup();

        cpu.pc = 0x1000;
        mem.data[0x1000] = Cpu::NOP_IMPLIED;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.pc, 0x1001);

        mem.data[0x1001] = Cpu::NOP_IMPLIED;
        cpu.execute(2, &mut mem);

        assert_eq!(cpu.pc, 0x1002);
    }

    #[test]
    fn test_rti_implied() {
        let (mut cpu, mut mem) = setup();

        cpu.sp = 0xFC;
        mem.data[0x0100 + 0xFD] = 0b1010_0101;
        mem.data[0x0100 + 0xFE] = 0x34;
        mem.data[0x0100 + 0xFF] = 0x12;
        mem.data[0x2000] = Cpu::RTI_IMPLIED;
        cpu.pc = 0x2000;
        cpu.execute(6, &mut mem);

        assert_eq!(cpu.pc, 0x1234);
        assert_eq!(cpu.n, 1);
        assert_eq!(cpu.v, 0);
        assert_eq!(cpu.b, 0);
        assert_eq!(cpu.d, 0);
        assert_eq!(cpu.i, 1);
        assert_eq!(cpu.z, 0);
        assert_eq!(cpu.c, 1);
    }
}