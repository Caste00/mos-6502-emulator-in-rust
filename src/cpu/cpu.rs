use crate::memory::memory::Memory;

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

    pub fn fetch_byte(&mut self, memory: &Memory) -> u8 {
        let data = memory.data[self.pc];
        self.pc += 1;
        data
    }

    pub fn fetch_word(&mut self, memory: &Memory) -> u16 {
        let first_byte = memory.data[self.pc];
        self.pc += 1;
        let second_byte = memory.data[self.pc];
        self.pc += 1;
        (second_byte as u16) << 8 | first_byte as u16
    }

    pub fn read_byte(&mut self, memory: &Memory, address: u16) -> u8 {
        let address = address as usize;
        let data = memory.data[address];
        data
    }

    pub fn read_word(&mut self, memory: &Memory, address:u16) -> u16 {
        let first_byte = memory.data[address as usize];
        let second_byte = memory.data[(address + 1) as usize];
        (second_byte as u16) << 8 | first_byte as u16
    }

    pub fn push_on_stack(&mut self, memory: &mut Memory, value: u8) {
        memory.data[0x0100 + self.sp as usize] = value;
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn pop_from_stack(&mut self, memory: &mut Memory) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        memory.data[0x0100 + self.sp as usize]
    }

    pub fn z_n_register_a_set_status(&mut self) {
        self.z = (self.a == 0) as u8;
        self.n = ((self.a & 0b1000_0000) > 0) as u8;
    }

    pub fn z_n_register_x_set_status(&mut self) {
        self.z = (self.x == 0) as u8;
        self.n = ((self.x & 0b1000_0000) > 0) as u8;
    }

    pub fn z_n_register_y_set_status(&mut self) {
        self.z = (self.y == 0) as u8;
        self.n = ((self.y & 0b1000_0000) > 0) as u8;
    }

    pub fn asl_set_status(&mut self, original: u8, result: u8) {
        self.c = ((original & 0b1000_0000) != 0) as u8; 
        self.z = (self.a == 0) as u8;
        self.n = ((result & 0b1000_0000) > 0) as u8;
    }

    pub fn and_set_status(&mut self) {
        self.z = (self.a == 0) as u8;
        self.n = ((self.a & 0b1000_0000) > 0) as u8;
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
                    let absolute_address = self.fetch_word(memory) as usize;
                    self.a = memory.data[absolute_address];
                    self.z_n_register_a_set_status();
                    cycle -= 4;
                },
                Self::LDA_ABSOLUTE_X => {
                    let absolute_address = self.fetch_word(memory).wrapping_add(self.x as u16) as usize;
                    if (absolute_address >> 8) != (absolute_address >> 8) {
                        cycle -= 1;
                    }
                    self.a = memory.data[absolute_address];
                    self.z_n_register_a_set_status();
                    cycle -= 4;
                },
                Self::LDA_ABSOLUTE_Y => {
                    let mut absolute_address = self.fetch_word(memory) as usize;
                    if (absolute_address >> 8) != ((absolute_address + self.y as usize) >> 8) {
                        cycle -= 1;
                    }
                    absolute_address = absolute_address.wrapping_add(self.y as usize);
                    self.a = memory.data[absolute_address];
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
                    if (indirect_address >> 8) != ((indirect_address + self.y as usize) >> 8) {
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
                    if (absolute_address >> 8) != ((absolute_address + self.y as usize) >> 8) {
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
                    if (absolute_address >> 8) != ((absolute_address + self.x as usize) >> 8) {
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
                    self.and_set_status();
                    cycle -= 2;
                },
                Self::AND_ZERO_PAGE => {
                    let address = self.fetch_byte(memory) as usize;
                    self.a &= memory.data[address];
                    self.and_set_status();
                    cycle -= 3;
                },
                Self::AND_ZERO_PAGE_X => {
                    let mut address = self.fetch_byte(memory);
                    address = address.wrapping_add(self.x);
                    self.a &= memory.data[address as usize];
                    self.and_set_status();
                    cycle -= 4;
                },
                Self::AND_ABSOLUTE => {
                    let address = self.fetch_word(memory) as usize;
                    self.a &= memory.data[address];
                    self.and_set_status();
                    cycle -= 4;
                },
                Self::AND_ABSOLUTE_X => {
                    let mut address = self.fetch_word(memory);
                    address = address.wrapping_add(self.x as u16);
                    self.a &= memory.data[address as usize];
                    self.and_set_status();
                    cycle -= 4;
                },
                Self::AND_ABSOLUTE_Y => {
                    let mut address = self.fetch_word(memory);
                    address = address.wrapping_add(self.y as u16);
                    self.a &= memory.data[address as usize];
                    self.and_set_status();
                    cycle -= 4;
                },
                Self::AND_INDIRECT_X => {
                    let zero_page_address = self.fetch_byte(memory).wrapping_add(self.x) as u16;
                    let indirect_address = self.read_word(memory, zero_page_address) as usize;
                    self.a &= memory.data[indirect_address];
                    self.and_set_status();
                    cycle -= 6;
                },
                Self::AND_INDIRECT_Y => {
                    let zero_page_address = self.fetch_byte(memory) as u16;
                    let mut indirect_address = self.read_word(memory, zero_page_address) as usize;
                    if (indirect_address >> 8) != ((indirect_address + self.y as usize) >> 8) {
                        cycle -= 1;
                    }
                    indirect_address = indirect_address.wrapping_add(self.y as usize);
                    self.a &= memory.data[indirect_address];
                    cycle -= 5;
                },
                /*  Self::LDA_INDIRECT_Y => {
                        let zero_page_address = self.fetch_byte(memory);
                        let indirect_address = self.read_word(memory, zero_page_address as u16).wrapping_add(self.y as u16) as usize;
                        if (indirect_address >> 8) != (indirect_address >> 8) {
                            cycle -= 1;
                        }
                        println!("{}", indirect_address);
                        self.a = memory.data[indirect_address];
                        cycle -= 5;
                    },*/

                _ => {
                    println!("Errore, istruzione {} non riconosciuta", instruction);
                    break;
                }
            }
        }
    }
}