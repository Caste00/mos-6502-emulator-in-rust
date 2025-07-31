use crate::memory::memory::Memory;

// Userò, per accedere al bit più significativo, sia byte >> 7 che byte & 0b1000_0000. Fanno la stessa cosa

#[derive(Debug)]
pub struct Cpu {
    pub pc: usize,  // 16 bits program counter
    pub sp: u8,     // 8 bits stack pointer
    pub a: u8,      // 8 bits accumulator
    pub x: u8,      // 8 bits index register
    pub y: u8,      // 8 bits index register
    
    // potrei usare solo 8 bit e una maschera al posto di 7 variabili differenti
    pub n: u8,
    pub v: u8,
    pub b: u8,
    pub d: u8,
    pub i: u8,
    pub z: u8,
    pub c: u8,
}

impl Cpu {
    pub const JSR_ABSOLUTE: u8 = 0x20;
    pub const RST_IMPLIED: u8 = 0x60;
    pub const JMP_ABSOLUTE: u8 = 0x4C;
    pub const JMP_INDIRECT: u8 = 0x6c;
    pub const LDA_IMMEDIATE: u8 = 0xA9;
    pub const LDA_ZERO_PAGE: u8 = 0xA5;
    pub const LDA_ZERO_PAGE_X: u8 = 0xB5;
    pub const LDA_ABSOLUTE: u8 = 0xAD;
    pub const LDA_ABSOLUTE_X: u8 = 0xBD;
    pub const LDA_ABSOLUTE_Y: u8 = 0xB9;
    pub const LDA_INDIRECT_X: u8 = 0xA1;
    pub const LDA_INDIRECT_Y: u8 = 0xB1;
    pub const LDX_IMMEDIATE: u8 = 0xA2;
    pub const LDX_ZERO_PAGE: u8 = 0xA6;
    pub const LDX_ZERO_PAGE_Y: u8 = 0xB6;
    pub const LDX_ABSOLUTE: u8 = 0xAE;
    pub const LDX_ABSOLUTE_Y: u8 = 0xBE;
    pub const LDY_IMMEDIATE: u8 = 0xA0;
    pub const LDY_ZERO_PAGE: u8 = 0xA4;
    pub const LDY_ZERO_PAGE_X: u8 = 0xB4;
    pub const LDY_ABSOLUTE: u8 = 0xAC;
    pub const LDY_ABSOLUTE_X: u8 = 0xBC;
    pub const STA_ZERO_PAGE: u8 = 0x85;
    pub const STA_ZERO_PAGE_X: u8 = 0x95;
    pub const STA_ABSOLUTE: u8 = 0x8D;
    pub const STA_ABSOLUTE_X: u8 = 0x9D;
    pub const STA_ABSOLUTE_Y: u8 = 0x99;
    pub const STA_INDIRECT_X: u8 = 0x81;
    pub const STA_INDIRECT_Y: u8 = 0x91;
    pub const STX_ZERO_PAGE: u8 = 0x86;
    pub const STX_ZERO_PAGE_Y: u8 = 0x96;
    pub const STX_ABSOLUTE: u8 = 0x8E;
    pub const STY_ZERO_PAGE: u8 = 0x84;
    pub const STY_ZERO_PAGE_X: u8 = 0x94;
    pub const STY_ABSOLUTE: u8 = 0x8C;
    pub const TAX_IMPLIED: u8 = 0xAA;
    pub const TXA_IMPLIED: u8 = 0x8A;
    pub const TAY_IMPLIED: u8 = 0xA8;
    pub const TYA_IMPLIED: u8 = 0x98;
    pub const TXS_IMPLICIT: u8 = 0x9A;   
    pub const TSX_IMPLIED: u8 = 0xBA;
    pub const ASL_ACCUMULATOR: u8 = 0x0A;
    pub const ASL_ZERO_PAGE: u8 = 0x06;
    pub const ASL_ZERO_PAGE_X: u8 = 0x16;
    pub const ASL_ABSOLUTE: u8 = 0x0E;
    pub const ASL_ABSOLUTE_X: u8 = 0x1E;
    pub const AND_IMMEDIATE: u8 = 0x29;
    pub const AND_ZERO_PAGE: u8 = 0x25;
    pub const AND_ZERO_PAGE_X: u8 = 0x35;
    pub const AND_ABSOLUTE: u8 = 0x2D;
    pub const AND_ABSOLUTE_X: u8 = 0x3D;
    pub const AND_ABSOLUTE_Y: u8 = 0x39;
    pub const AND_INDIRECT_X: u8 = 0x21;
    pub const AND_INDIRECT_Y: u8 = 0x31;
    pub const EOR_IMMEDIATE: u8 = 0x49;
    pub const EOR_ZERO_PAGE: u8 = 0x45;
    pub const EOR_ZERO_PAGE_X: u8 = 0x55;
    pub const EOR_ABSOLUTE: u8 = 0x4D;
    pub const EOR_ABSOLUTE_X: u8 = 0x5D;
    pub const EOR_ABSOLUTE_Y: u8 = 0x59;
    pub const EOR_INDIRECT_X: u8 = 0x41;
    pub const EOR_INDIRECT_Y: u8 = 0x51;
    pub const ORA_IMMEDIATE: u8 = 0x09;
    pub const ORA_ZERO_PAGE: u8 = 0x05;
    pub const ORA_ZERO_PAGE_X: u8 = 0x15;
    pub const ORA_ABSOLUTE: u8 = 0x0D;
    pub const ORA_ABSOLUTE_X: u8 = 0x1D;
    pub const ORA_ABSOLUTE_Y: u8 = 0x19;
    pub const ORA_INDIRECT_X: u8 = 0x01;
    pub const ORA_INDIRECT_Y: u8 = 0x11;
    pub const BIT_ZERO_PAGE: u8 = 0x24;
    pub const BIT_ABSOLUTE: u8 = 0x2C;
    pub const BBC_RELATIVE: u8 = 0x90;
    pub const BCS_RELATIVE: u8 = 0xB0;
    pub const BEQ_RELATIVE: u8 = 0xF0;
    pub const BNE_RELATIVE: u8 = 0xD0;
    pub const BMI_RELATIVE: u8 = 0x30;
    pub const BPL_RELATIVE: u8 = 0x10;
    pub const BVS_RELATIVE: u8 = 0x70;
    pub const BVC_RELATIVE: u8 = 0x80;
    pub const CLC_IMPLIED: u8 = 0x18;
    pub const CLD_IMPLIED: u8 = 0xD8;
    pub const CLI_IMPLIED: u8 = 0x58;
    pub const CLV_IMPLIED: u8 = 0xB8;
    pub const SEC_IMPLIED: u8 = 0x38;
    pub const SED_IMPLIED: u8 = 0xF8;
    pub const SEI_IMPLIED: u8 = 0x78;
    pub const PHA_IMPLIED: u8 = 0x48;
    pub const PHP_IMPLIED: u8 = 0x08;
    pub const PLA_IMPLIED: u8 = 0x68;
    pub const PLP_IMPLIED: u8 = 0x28;
    pub const ADC_IMMEDIATE: u8 = 0x69;
    pub const ADC_ZERO_PAGE: u8 = 0x65;
    pub const ADC_ZERO_PAGE_X: u8 = 0x75;
    pub const ADC_ABSOLUTE: u8 = 0x6D;
    pub const ADC_ABSOLUTE_X: u8 = 0x7D;
    pub const ADC_ABSOLUTE_Y: u8 = 0x79;
    pub const ADC_INDIRECT_X: u8 = 0x61;

    pub fn new() -> Self {
        Self {
            pc: 0xFFFC,     // durante il reset legge i due byte all'indirizzo FFFC e FFFD, in little endian corrispondono al valore iniziale di reset
            sp: 0xFD,       // lo stack pointer parte dal massimo e viene decrementato (cresce verso il basso)
            a: 0, x: 0, y: 0,
            n: 0, v: 0, b: 0, d: 0, i: 0, z: 0, c: 0
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0xFFFC;        // sarà da modificare 
        self.sp = 0xFD;
        self.a = 0; 
        self.x = 0;
        self.y = 0;
        self.n = 0;
        self.v = 0;
        self.b = 0;
        self.d = 0;
        self.i = 1;
        self.z = 0;
        self.c = 0;
    }

    fn fetch_byte(&mut self, memory: &Memory) -> u8 {
        let data = memory.data[self.pc];
        self.pc += 1;
        data
    }

    fn fetch_word(&mut self, memory: &Memory) -> u16 {
        let first_byte = memory.data[self.pc];
        self.pc += 1;
        let second_byte = memory.data[self.pc];
        self.pc += 1;
        (second_byte as u16) << 8 | first_byte as u16
    }

    fn read_byte(&mut self, memory: &Memory, address: u16) -> u8 {
        let address = address as usize;
        let data = memory.data[address];
        data
    }

    fn read_word(&mut self, memory: &Memory, address:u16) -> u16 {
        let first_byte = memory.data[address as usize];
        let second_byte = memory.data[(address + 1) as usize];
        (second_byte as u16) << 8 | first_byte as u16
    }

    fn push_on_stack(&mut self, memory: &mut Memory, value: u8) {
        memory.data[0x0100 + self.sp as usize] = value;
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pop_from_stack(&mut self, memory: &mut Memory) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        memory.data[0x0100 + self.sp as usize]
    }

    fn is_page_crossed(&mut self, absolute_address: usize, register: u8) -> bool {
        (absolute_address >> 8) != (absolute_address.wrapping_add(register as usize) >> 8)
    }

    fn z_n_register_a_set_status(&mut self) {
        self.z = (self.a == 0) as u8;
        self.n = ((self.a & 0b1000_0000) > 0) as u8;
    }

    fn z_n_register_x_set_status(&mut self) {
        self.z = (self.x == 0) as u8;
        self.n = ((self.x & 0b1000_0000) > 0) as u8;
    }

    fn z_n_register_y_set_status(&mut self) {
        self.z = (self.y == 0) as u8;
        self.n = ((self.y & 0b1000_0000) > 0) as u8;
    }

    fn asl_set_status(&mut self, original: u8, result: u8) {
        self.c = ((original & 0b1000_0000) != 0) as u8; 
        self.z = (self.a == 0) as u8;
        self.n = ((result & 0b1000_0000) > 0) as u8;
    }

    fn end_or_set_status(&mut self) {
        self.z = (self.a == 0) as u8;
        self.n = ((self.a & 0b1000_0000) > 0) as u8;
    }

    fn bit_set_status(&mut self, and_result: u8, memory_value: u8) {
        self.z = (and_result == 0) as u8;
        self.n = ((memory_value & 0b1000_0000) > 0) as u8;
        self.v = ((memory_value & 0b0100_0000) > 0) as u8;
    }

    fn adc_sbc_set_status(&mut self, operand: u8, a: u8, sum: u16) {
        self.c = (sum > 0xFF) as u8;
        self.z = (self.a == 0) as u8;
        self.n = ((self.a & 0b1000_0000) > 0) as u8;
        self.v = ((operand >> 7) == (a >> 7) && (self.n) != (a >> 7)) as u8;
    }

    pub fn execute(&mut self, tick: u32, memory: &mut Memory) {
        let mut cycle = tick;

        while cycle > 0 {
            let instruction = self.fetch_byte(memory);

            match instruction {
                Self::LDA_IMMEDIATE => {
                    self.a = self.fetch_byte(memory);
                    self.z_n_register_a_set_status();
                    cycle -= 2;
                },
                Self::LDA_ZERO_PAGE => {
                    let zero_page_address = self.fetch_byte(memory) as u16;
                    self.a = self.read_byte(memory, zero_page_address);
                    self.z_n_register_a_set_status();
                    cycle -= 3
                },
                Self::LDA_ZERO_PAGE_X => {
                    let zero_page_address = self.fetch_byte(memory).wrapping_add(self.x) as u16;
                    self.a = self.read_byte(memory, zero_page_address);
                    self.z_n_register_a_set_status();
                    cycle -= 4;
                },
                Self::LDA_ABSOLUTE => {
                    let address = self.fetch_word(memory) as usize;
                    self.a = memory.data[address];
                    self.z_n_register_a_set_status();
                    cycle -= 4;
                },
                Self::LDA_ABSOLUTE_X => {
                    let mut address = self.fetch_word(memory) as usize;
                    if self.is_page_crossed(address, self.x) {
                        cycle -= 1;
                    }
                    address = address.wrapping_add(self.x as usize);
                    self.a = memory.data[address];
                    self.z_n_register_a_set_status();
                    cycle -= 4;
                },
                Self::LDA_ABSOLUTE_Y => {
                    let mut address = self.fetch_word(memory) as usize;
                    if self.is_page_crossed(
                        address, self.y) {
                        cycle -= 1;
                    }
                    address = address.wrapping_add(self.y as usize);
                    self.a = memory.data[address];
                    self.z_n_register_a_set_status();
                    cycle -= 4;
                },
                Self::LDA_INDIRECT_X => {
                    let table_address = self.fetch_byte(memory).wrapping_add(self.x);
                    let indirect_address = self.read_word(memory, table_address as u16) as usize;
                    self.a = memory.data[indirect_address];
                    self.z_n_register_a_set_status();
                    cycle -= 6;
                },
                Self::LDA_INDIRECT_Y => {
                    let zero_page_address = self.fetch_byte(memory) as u16;
                    let mut indirect_address = self.read_word(memory, zero_page_address) as usize;
                    if self.is_page_crossed(indirect_address, self.y) {
                        cycle -= 1;
                    }
                    indirect_address = indirect_address.wrapping_add(self.y as usize);
                    self.a = memory.data[indirect_address];
                    cycle -= 5;
                },
                Self::JSR_ABSOLUTE => {
                    let subroutine_address = self.fetch_word(memory);
                    self.push_on_stack(memory, ((self.pc - 1) >> 8) as u8);             // byte alto
                    self.push_on_stack(memory, ((self.pc - 1) & 0xFF) as u8);           // byte basso
                    self.pc = subroutine_address as usize;
                    cycle -= 6;
                },
                Self::RST_IMPLIED => {
                    let first_byte = self.pop_from_stack(memory) as usize;
                    let second_byte = self.pop_from_stack(memory) as usize;
                    self.pc = ((second_byte << 8) | first_byte) + 1;
                    cycle -= 6;
                },
                Self::JMP_ABSOLUTE => {
                    self.pc = self.fetch_word(memory) as usize;
                    cycle -= 3;
                },
                Self::JMP_INDIRECT => {
                    let jmp_address = self.fetch_word(memory) as usize;
                    let fist_byte = memory.data[jmp_address] as usize;
                    let second_byte = memory.data[jmp_address + 1] as usize;
                    self.pc = (second_byte << 8) | fist_byte;
                    cycle -= 5;
                },
                Self::LDX_IMMEDIATE => {
                    self.x = self.fetch_byte(memory);
                    self.z_n_register_x_set_status();
                    cycle -= 2;
                },
                Self::LDX_ZERO_PAGE => {
                    let zero_page_address = self.fetch_byte(memory) as u16;
                    self.x = self.read_byte(memory, zero_page_address);
                    self.z_n_register_x_set_status();
                    cycle -= 3;
                },
                Self::LDX_ZERO_PAGE_Y => {
                    let zero_page_address = self.fetch_byte(memory).wrapping_add(self.y) as u16;
                    self.x = self.read_byte(memory, zero_page_address );
                    self.z_n_register_x_set_status();
                    cycle -= 4;
                },
                Self::LDX_ABSOLUTE => {
                    let absolute_address = self.fetch_word(memory) as usize;
                    self.x = memory.data[absolute_address];
                    self.z_n_register_x_set_status();
                    cycle -= 4;
                },
                Self::LDX_ABSOLUTE_Y => {
                    let mut absolute_address = self.fetch_word(memory) as usize;
                    if self.is_page_crossed(absolute_address, self.y) {
                        cycle -= 1;
                    }
                    absolute_address = absolute_address.wrapping_add(self.y as usize);
                    self.x = memory.data[absolute_address];
                    self.z_n_register_x_set_status();
                    cycle -= 4;
                },
                Self::LDY_IMMEDIATE => {
                    self.y = self.fetch_byte(memory);
                    self.z_n_register_y_set_status();
                    cycle -= 2;
                },
                Self::LDY_ZERO_PAGE => {
                    let zero_page_address = self.fetch_byte(memory) as u16;
                    self.y = self.read_byte(memory, zero_page_address);
                    self.z_n_register_y_set_status();
                    cycle -= 3;
                },
                Self::LDY_ZERO_PAGE_X => {
                    let zero_page_address = self.fetch_byte(memory).wrapping_add(self.x) as u16;
                    self.y = self.read_byte(memory, zero_page_address);
                    self.z_n_register_y_set_status();
                    cycle -= 4;
                },
                Self::LDY_ABSOLUTE => {
                    let absolute_address = self.fetch_word(memory) as usize;
                    self.y = memory.data[absolute_address];
                    self.z_n_register_y_set_status();
                    cycle -= 4;
                },
                Self::LDY_ABSOLUTE_X => {
                    let mut absolute_address = self.fetch_word(memory) as usize;
                    if self.is_page_crossed(absolute_address, self.x) {
                        cycle -= 1;
                    }
                    absolute_address = absolute_address.wrapping_add(self.x as usize);
                    self.y = memory.data[absolute_address];
                    self.z_n_register_y_set_status();
                    cycle -= 4;
                },
                Self::STA_ZERO_PAGE => {
                    let address = self.fetch_byte(memory) as usize;
                    memory.data[address] = self.a;
                    cycle -= 3;
                },
                Self::STA_ZERO_PAGE_X => {
                    let address = self.fetch_byte(memory).wrapping_add(self.x) as usize;
                    memory.data[address] = self.a;
                    cycle -= 4;
                },
                Self::STA_ABSOLUTE => {
                    let address = self.fetch_word(memory) as usize;
                    memory.data[address] = self.a;
                    cycle -= 4;
                },
                Self::STA_ABSOLUTE_X => {
                    let address = self.fetch_word(memory).wrapping_add(self.x as u16) as usize;
                    memory.data[address] = self.a;
                    cycle -= 5;
                },
                Self::STA_ABSOLUTE_Y => {
                    let address = self.fetch_word(memory).wrapping_add(self.y as u16) as usize;
                    memory.data[address] = self.a;
                    cycle -= 5;
                },
                Self::STA_INDIRECT_X => {
                    let table_address = self.fetch_byte(memory).wrapping_add(self.x) as u16;
                    let indirect_address = self.read_word(memory, table_address) as usize;
                    memory.data[indirect_address] = self.a;
                    cycle -= 6;
                },
                Self::STA_INDIRECT_Y => {
                    let table_address = self.fetch_byte(memory) as u16;
                    let indirect_address = self.read_word(memory, table_address).wrapping_add(self.y as u16) as usize;
                    memory.data[indirect_address] = self.a;
                    cycle -= 6;
                },
                Self::STX_ZERO_PAGE => {
                    let address = self.fetch_byte(memory) as usize;
                    memory.data[address] = self.x;
                    cycle -= 3;
                },
                Self::STX_ZERO_PAGE_Y => {
                    let address = self.fetch_byte(memory).wrapping_add(self.y) as usize;
                    memory.data[address] = self.x;
                    cycle -= 4;
                },
                Self::STX_ABSOLUTE => {
                    let address = self.fetch_word(memory) as usize;
                    memory.data[address] = self.x;
                    cycle -= 4;
                },
                Self::STY_ZERO_PAGE => {
                    let address = self.fetch_byte(memory) as usize;
                    memory.data[address] = self.y;
                    cycle -= 3;
                }, 
                Self::STY_ZERO_PAGE_X => {
                    let address = self.fetch_byte(memory).wrapping_add(self.x) as usize;
                    memory.data[address] = self.y;
                    cycle -= 4;
                }, 
                Self::STY_ABSOLUTE => {
                    let address = self.fetch_word(memory) as usize;
                    memory.data[address] = self.y;
                    cycle -= 4;
                },
                Self::TAX_IMPLIED => {
                    self.x = self.a;
                    self.z_n_register_x_set_status();
                    cycle -= 2;
                },
                Self::TXA_IMPLIED => {
                    self.a = self.x;
                    self.z_n_register_a_set_status();
                    cycle -= 2;
                },
                Self::TAY_IMPLIED => {
                    self.y = self.a;
                    self.z_n_register_y_set_status();
                    cycle -= 2;
                }
                Self::TYA_IMPLIED => {
                    self.a = self.y;
                    self.z_n_register_a_set_status();
                    cycle -= 2;
                },
                Self::TXS_IMPLICIT => {
                    self.sp = self.x;
                    cycle -= 2;
                },
                Self::TSX_IMPLIED => {
                    self.x = self.sp;
                    self.z_n_register_x_set_status();
                    cycle -= 2;
                },
                Self::ASL_ACCUMULATOR => {
                    let original = self.a;
                    self.a = self.a.wrapping_shl(1);
                    self.asl_set_status(original, self.a);
                    cycle -= 2;
                }, 
                Self::ASL_ZERO_PAGE => {
                    let address = self.fetch_byte(memory) as usize;
                    let original = memory.data[address];
                    memory.data[address] = memory.data[address].wrapping_shl(1);
                    self.asl_set_status(original, memory.data[address]);
                    cycle -= 5;
                }, 
                Self::ASL_ZERO_PAGE_X => {
                    let address = self.fetch_byte(memory).wrapping_add(self.x) as usize;
                    let original = memory.data[address];
                    memory.data[address] = memory.data[address].wrapping_shl(1);
                    self.asl_set_status(original, memory.data[address]);
                    cycle -= 6;
                }, 
                Self::ASL_ABSOLUTE => {
                    let address = self.fetch_word(memory) as usize;
                    let original = memory.data[address];
                    memory.data[address] = memory.data[address].wrapping_shl(1);
                    self.asl_set_status(original, memory.data[address]);
                    cycle -= 6;
                }, 
                Self::ASL_ABSOLUTE_X => {
                    let address = self.fetch_word(memory).wrapping_add(self.x as u16) as usize;
                    let original = memory.data[address];
                    memory.data[address] = memory.data[address].wrapping_shl(1);
                    self.asl_set_status(original, memory.data[address]);
                    cycle -= 7;
                },
                Self::AND_IMMEDIATE => {
                    let data = self.fetch_byte(memory);
                    self.a &= data;
                    self.end_or_set_status();
                    cycle -= 2;
                },
                Self::AND_ZERO_PAGE => {
                    let address = self.fetch_byte(memory) as usize;
                    self.a &= memory.data[address];
                    self.end_or_set_status();
                    cycle -= 3;
                },
                Self::AND_ZERO_PAGE_X => {
                    let mut address = self.fetch_byte(memory);
                    address = address.wrapping_add(self.x);
                    self.a &= memory.data[address as usize];
                    self.end_or_set_status();
                    cycle -= 4;
                },
                Self::AND_ABSOLUTE => {
                    let address = self.fetch_word(memory) as usize;
                    self.a &= memory.data[address];
                    self.end_or_set_status();
                    cycle -= 4;
                },
                Self::AND_ABSOLUTE_X => {
                    let mut address = self.fetch_word(memory);
                    address = address.wrapping_add(self.x as u16);
                    self.a &= memory.data[address as usize];
                    self.end_or_set_status();
                    cycle -= 4;
                },
                Self::AND_ABSOLUTE_Y => {
                    let mut address = self.fetch_word(memory);
                    address = address.wrapping_add(self.y as u16);
                    self.a &= memory.data[address as usize];
                    self.end_or_set_status();
                    cycle -= 4;
                },
                Self::AND_INDIRECT_X => {
                    let zero_page_address = self.fetch_byte(memory).wrapping_add(self.x) as u16;
                    let indirect_address = self.read_word(memory, zero_page_address) as usize;
                    self.a &= memory.data[indirect_address];
                    self.end_or_set_status();
                    cycle -= 6;
                },
                Self::AND_INDIRECT_Y => {
                    let zero_page_address = self.fetch_byte(memory) as u16;
                    let mut indirect_address = self.read_word(memory, zero_page_address) as usize;
                    if self.is_page_crossed(indirect_address, self.y) {
                        cycle -= 1;
                    }
                    indirect_address = indirect_address.wrapping_add(self.y as usize);
                    self.a &= memory.data[indirect_address];
                    cycle -= 5;
                },
                Self::ORA_IMMEDIATE => {
                    let data = self.fetch_byte(memory);
                    self.a |= data;
                    self.end_or_set_status();
                    cycle -= 2;
                },
                Self::ORA_ZERO_PAGE => {
                    let address = self.fetch_byte(memory) as usize;
                    self.a |= memory.data[address];
                    self.end_or_set_status();
                    cycle -= 3;
                },
                Self::ORA_ZERO_PAGE_X => {
                    let address = self.fetch_byte(memory).wrapping_add(self.x) as usize;
                    self.a |= memory.data[address];
                    self.end_or_set_status();
                    cycle -= 4;
                },
                Self::ORA_ABSOLUTE => {
                    let address = self.fetch_word(memory) as usize;
                    self.a |= memory.data[address];
                    self.end_or_set_status();
                    cycle -= 4;
                },
                Self::ORA_ABSOLUTE_X => {
                    let mut address = self.fetch_word(memory) as usize;
                    if self.is_page_crossed(address, self.y) {
                        cycle -= 1;
                    }
                    address = address.wrapping_add(self.x as usize);
                    self.a |= memory.data[address];
                    self.end_or_set_status();
                    cycle -= 4;
                },
                Self::ORA_ABSOLUTE_Y => {
                    let mut address = self.fetch_word(memory) as usize;
                    if self.is_page_crossed(address, self.y) {
                        cycle -= 1;
                    }
                    address = address.wrapping_add(self.y as usize);
                    self.a |= memory.data[address];
                    self.end_or_set_status();
                    cycle -= 4;
                },
                Self::ORA_INDIRECT_X => {
                    let zero_page_address = self.fetch_byte(memory).wrapping_add(self.x) as u16;
                    let indirect_address = self.read_word(memory, zero_page_address) as usize;
                    self.a |= memory.data[indirect_address];
                    self.end_or_set_status();
                    cycle -= 6;
                },
                Self::ORA_INDIRECT_Y => {
                    let zero_page_address = self.fetch_byte(memory) as u16;
                    let mut indirect_address = self.read_word(memory, zero_page_address) as usize;
                    if self.is_page_crossed(indirect_address, self.y) {
                        cycle -= 1;
                    }
                    indirect_address = indirect_address.wrapping_add(self.y as usize);
                    self.a |= memory.data[indirect_address];
                    self.end_or_set_status();
                    cycle -= 5;
                },
                Self::EOR_IMMEDIATE => {
                    let data = self.fetch_byte(memory);
                    self.a ^= data;
                    self.end_or_set_status();
                    cycle -= 2;
                },
                Self::EOR_ZERO_PAGE => {
                    let address = self.fetch_byte(memory) as usize;
                    self.a ^= memory.data[address];
                    self.end_or_set_status();
                    cycle -= 3;
                },
                Self::EOR_ZERO_PAGE_X => {
                    let address = self.fetch_byte(memory).wrapping_add(self.x) as usize;
                    self.a ^= memory.data[address];
                    self.end_or_set_status();
                    cycle -= 4;
                },
                Self::EOR_ABSOLUTE => {
                    let address = self.fetch_word(memory) as usize;
                    self.a ^= memory.data[address];
                    self.end_or_set_status();
                    cycle -= 4;
                },
                Self::EOR_ABSOLUTE_X => {
                    let mut address = self.fetch_word(memory) as usize;
                    if self.is_page_crossed(address, self.y) {
                        cycle -= 1;
                    }
                    address = address.wrapping_add(self.x as usize);
                    self.a ^= memory.data[address];
                    self.end_or_set_status();
                    cycle -= 4;
                },
                Self::EOR_ABSOLUTE_Y => {
                    let mut address = self.fetch_word(memory) as usize;
                    if self.is_page_crossed(address, self.y) {
                        cycle -= 1;
                    }
                    address = address.wrapping_add(self.y as usize);
                    self.a ^= memory.data[address];
                    self.end_or_set_status();
                    cycle -= 4;
                },
                Self::EOR_INDIRECT_X => {
                    let zero_page_address = self.fetch_byte(memory).wrapping_add(self.x) as u16;
                    let indirect_address = self.read_word(memory, zero_page_address) as usize;
                    self.a ^= memory.data[indirect_address];
                    self.end_or_set_status();
                    cycle -= 6;
                },
                Self::EOR_INDIRECT_Y => {
                    let zero_page_address = self.fetch_byte(memory) as u16;
                    let mut indirect_address = self.read_word(memory, zero_page_address) as usize;
                    if self.is_page_crossed(indirect_address, self.y) {
                        cycle -= 1;
                    }
                    indirect_address = indirect_address.wrapping_add(self.y as usize);
                    self.a ^= memory.data[indirect_address];
                    self.end_or_set_status();
                    cycle -= 5;
                },
                Self::BIT_ZERO_PAGE => {
                    let address = self.fetch_byte(memory) as usize;
                    let and_result = self.a & memory.data[address];
                    self.bit_set_status(and_result, memory.data[address]);
                    cycle -= 3;
                },
                Self::BIT_ABSOLUTE => {
                    let address = self.fetch_word(memory) as usize;
                    let and_result = self.a & memory.data[address];
                    self.bit_set_status(and_result, memory.data[address]);
                    cycle -= 4;
                },
                Self::BBC_RELATIVE => {
                    let offset = self.fetch_byte(memory) as i8;
                    if self.c == 0 {
                        let new_pc = (self.pc as isize).wrapping_add(offset as isize) as usize;
                        if (self.pc >> 8) != (new_pc >> 8) {
                            cycle -= 1;
                        }
                        self.pc = new_pc;
                        cycle -= 1;
                    }
                    cycle -= 2;
                },
                Self::BCS_RELATIVE => {
                    let offset = self.fetch_byte(memory) as i8;
                    if self.c == 1 {
                        let new_pc = (self.pc as isize).wrapping_add(offset as isize) as usize;
                        if (self.pc >> 8) != (new_pc >> 8) {
                            cycle -= 1;
                        }
                        self.pc = new_pc;
                        cycle -= 1;
                    }
                    cycle -= 2;
                },
                Self::BEQ_RELATIVE => {
                    let offset = self.fetch_byte(memory) as i8;
                    if self.z == 1 {
                        let new_pc = (self.pc as isize).wrapping_add(offset as isize) as usize;
                        if (self.pc >> 8) != (new_pc >> 8) {
                            cycle -= 1;
                        }
                        self.pc = new_pc;
                        cycle -= 1;
                    }
                    cycle -= 2;
                },
                Self::BNE_RELATIVE => {
                    let offset = self.fetch_byte(memory) as i8;
                    if self.z == 0 {
                        let new_pc = (self.pc as isize).wrapping_add(offset as isize) as usize;
                        if (self.pc >> 8) != (new_pc >> 8) {
                            cycle -= 1;
                        }
                        self.pc = new_pc;
                        cycle -= 1;
                    }
                    cycle -= 2;
                },
                Self::BMI_RELATIVE => {
                    let offset = self.fetch_byte(memory) as i8;
                    if self.n == 1 {
                        let new_pc = (self.pc as isize).wrapping_add(offset as isize) as usize;
                        if (self.pc >> 8) != (new_pc >> 8) {
                            cycle -= 1;
                        }
                        self.pc = new_pc;
                        cycle -= 1;
                    }
                    cycle -= 2;
                },
                Self::BPL_RELATIVE => {
                    let offset = self.fetch_byte(memory) as i8;
                    if self.n == 0 {
                        let new_pc = (self.pc as isize).wrapping_add(offset as isize) as usize;
                        if (self.pc >> 8) != (new_pc >> 8) {
                            cycle -= 1;
                        }
                        self.pc = new_pc;
                        cycle -= 1;
                    }
                    cycle -= 2;
                },
                Self::BVS_RELATIVE => {
                    let offset = self.fetch_byte(memory) as i8;
                    if self.v == 1 {
                        let new_pc = (self.pc as isize).wrapping_add(offset as isize) as usize;
                        if (self.pc >> 8) != (new_pc >> 8) {
                            cycle -= 1;
                        }
                        self.pc = new_pc;
                        cycle -= 1;
                    }
                    cycle -= 2;
                },
                Self::BVC_RELATIVE => {
                    let offset = self.fetch_byte(memory) as i8;
                    if self.v == 0 {
                        let new_pc = (self.pc as isize).wrapping_add(offset as isize) as usize;
                        if (self.pc >> 8) != (new_pc >> 8) {
                            cycle -= 1;
                        }
                        self.pc = new_pc;
                        cycle -= 1;
                    }
                    cycle -= 2;
                },
                Self::CLC_IMPLIED => {
                    self.c = 0;
                    cycle -= 2;
                },
                Self::CLD_IMPLIED => {
                    self.d = 0;
                    cycle -= 2;
                },
                Self::CLI_IMPLIED => {
                    self.i = 0;
                    cycle -= 2;
                },
                Self::CLV_IMPLIED => {
                    self.v = 0;
                    cycle -= 2;
                },
                Self::SEC_IMPLIED => {
                    self.c = 1;
                    cycle -= 2;
                },
                Self::SED_IMPLIED => {
                    self.d = 1;
                    cycle -= 2;
                },
                Self::SEI_IMPLIED => {
                    self.i = 1;
                    cycle -= 2;
                },
                Self::PHA_IMPLIED => {
                    self.push_on_stack(memory, self.a);
                    cycle -= 3;
                },
                Self::PHP_IMPLIED => {
                    let mut status = 0b0010_0000;
                    status |= self.n << 7;
                    status |= self.v << 6;
                    status |= self.b << 4;
                    status |= self.d << 3;
                    status |= self.i << 2;
                    status |= self.z << 1;
                    status |= self.c;
                    self.push_on_stack(memory, status);
                    cycle -= 3;
                },
                Self::PLA_IMPLIED => {
                    self.a = self.pop_from_stack(memory);
                    self.z_n_register_a_set_status();
                    cycle -= 4;
                },
                Self::PLP_IMPLIED => {
                    let status = self.pop_from_stack(memory);
                    self.n = (status >> 7) & 1;
                    self.v = (status >> 6) & 1;
                    self.b = (status >> 4) & 1;
                    self.d = (status >> 3) & 1;
                    self.i = (status >> 2) & 1;
                    self.z = (status >> 1) & 1;
                    self.c = status & 1;
                    cycle -= 4;
                },
                Self::ADC_IMMEDIATE => {
                    let operand = self.fetch_byte(memory);
                    let a = self.a;
                    let sum = a as u16 + operand as u16 + self.c as u16;
                    self.a = sum as u8;
                    self.adc_sbc_set_status(operand, a, sum);
                    cycle -= 2;
                },
                Self::ADC_ZERO_PAGE => {
                    let address = self.fetch_byte(memory) as usize;
                    let operand = memory.data[address];
                    let a = self.a;
                    let sum = a as u16 + operand as u16 + self.c as u16;
                    self.a = sum as u8;
                    self.adc_sbc_set_status(operand, a, sum);
                    cycle -= 3;
                },
                Self::ADC_ZERO_PAGE_X => {
                    let address = self.fetch_byte(memory).wrapping_add(self.x) as usize;
                    let operand = memory.data[address];
                    let a = self.a;
                    let sum = a as u16 + operand as u16 + self.c as u16;
                    self.a = sum as u8;
                    self.adc_sbc_set_status(operand, a, sum);
                    cycle -= 4;
                },
                Self::ADC_ABSOLUTE => {
                    let address = self.fetch_word(memory) as usize;
                    let operand = memory.data[address];
                    let a = self.a;
                    let sum = a as u16 + operand as u16 + self.c as u16;
                    self.a = sum as u8;
                    self.adc_sbc_set_status(operand, a, sum);
                    cycle -= 4;
                },
                Self::ADC_ABSOLUTE_X => {
                    let address = self.fetch_word(memory) as usize;
                    if self.is_page_crossed(address, self.x) {
                        cycle -= 1;
                    }
                    let operand = memory.data[address.wrapping_add(self.x as usize)];
                    let a = self.a;
                    let sum = a as u16 + operand as u16 + self.c as u16;
                    self.a = sum as u8;
                    self.adc_sbc_set_status(operand, a, sum);
                    cycle -= 4;
                },
                Self::ADC_ABSOLUTE_Y => {
                    let address = self.fetch_word(memory) as usize;
                    if self.is_page_crossed(address, self.y) {
                        cycle -= 1;
                    }
                    let operand = memory.data[address.wrapping_add(self.y as usize)];
                    let a = self.a;
                    let sum = a as u16 + operand as u16 + self.c as u16;
                    self.a = sum as u8;
                    self.adc_sbc_set_status(operand, a, sum);
                    cycle -= 4;
                },
                Self::ADC_INDIRECT_X => {
                    let zero_page_address = self.fetch_byte(memory).wrapping_add(self.x) as u16;
                    let indirect_address = self.read_word(memory, zero_page_address) as usize;
                    let operand = memory.data[indirect_address];
                    let a = self.a;
                    let sum = a as u16 + operand as u16 + self.c as u16;
                    self.a = sum as u8;
                    self.adc_sbc_set_status(operand, a, sum);
                    cycle -= 6;
                },
                _ => {
                    println!("Error, instruction {} not recognized", instruction);
                    break;
                }
            }
        }
    }
}