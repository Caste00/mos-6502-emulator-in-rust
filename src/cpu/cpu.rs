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
    pub const LDA_IMMEDIATE: u8 = 0x9A;
    pub const LDA_ZERO_PAGE: u8 = 0xA5;
    pub const LDA_ZERO_PAGE_X: u8 = 0xB5;
    pub const LDA_ABSOLUTE: u8 = 0xAD;
    pub const LDA_ABSOLUTE_X: u8 = 0xBD;
    pub const LDA_ABSOLUTE_Y: u8 = 0xB9;
    pub const LDA_INDIRECT_X: u8 = 0xA1;
    pub const LDA_INDIRECT_Y: u8 = 0xB1;
    pub const JSR_ABSOLUTE: u8 = 0x20;
    pub const JMP_ABSOLUTE: u8 = 0x4C;

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
        self.sp += self.sp.wrapping_add(1);
        memory.data[0x0100 + self.pc as usize]
    }

    pub fn lda_set_status(&mut self) {
        self.z = (self.a == 0) as u8;
        self.n = ((self.a & 0b10000000) > 0) as u8;
    }

    pub fn execute(&mut self, tick: u32, memory: &mut Memory) {
        let mut cycle = tick;

        while cycle > 0 {
            let instruction = self.fetch_byte(memory);

            match instruction {
                Self::LDA_IMMEDIATE => {
                    let value = self.fetch_byte(memory);
                    self.a = value;
                    self.lda_set_status();
                    cycle -= 2;
                },
                Self::LDA_ZERO_PAGE => {
                    let zero_page_address = self.fetch_byte(memory);
                    self.a = self.read_byte(memory, zero_page_address as u16);
                    self.lda_set_status();
                    cycle -= 3
                },
                Self::LDA_ZERO_PAGE_X => {
                    let zero_page_address = self.fetch_byte(memory);
                    let (address, _) = zero_page_address.overflowing_add(self.x);
                    self.a = self.read_byte(memory, address as u16);
                    self.lda_set_status();
                    cycle -= 4;
                },
                Self::LDA_ABSOLUTE => {
                    let absolute_address = self.fetch_word(memory);
                    self.a = memory.data[absolute_address as usize];
                    self.lda_set_status();
                    cycle -= 4;
                },
                Self::LDA_ABSOLUTE_X => {
                    let absolute_address = self.fetch_word(memory);
                    let address = absolute_address.wrapping_add(self.x as u16);
                    if (absolute_address >> 8) != (address >> 8) {
                        cycle -= 1;
                    }
                    self.a = memory.data[address as usize];
                    self.lda_set_status();
                    cycle -= 4;
                },
                Self::LDA_ABSOLUTE_Y => {
                    let absolute_address = self.fetch_word(memory);
                    let address = absolute_address.wrapping_add(self.y as u16);
                    if (absolute_address >> 8) != (address >> 8) {
                        cycle -= 1;
                    }
                    self.a = memory.data[address as usize];
                    self.lda_set_status();
                    cycle -= 4;
                },
                Self::LDA_INDIRECT_X => {
                    let mut table_address = self.fetch_byte(memory);
                    table_address = table_address.wrapping_add(self.x);
                    let indirect_address = self.read_word(memory, table_address as u16);
                    self.a = memory.data[indirect_address as usize];
                    self.lda_set_status();
                    cycle -= 6;
                },
                Self::LDA_INDIRECT_Y => {
                    let zero_page_address = self.fetch_byte(memory);
                    let indirect_address = self.read_word(memory, zero_page_address as u16);
                    let address = indirect_address.wrapping_add(self.y as u16);
                    if (address >> 8) != (indirect_address >> 8) {
                        cycle -= 1;
                    }
                    println!("{}", address);
                    self.a = memory.data[address as usize];
                    cycle -= 5;
                },
                Self::JSR_ABSOLUTE => {
                    let subroutine_address = self.fetch_word(memory);
                    self.push_on_stack(memory, ((self.pc - 1) >> 8) as u8); // byte alto
                    self.push_on_stack(memory, (self.pc - 1) as u8);        // byte basso
                    self.pc = subroutine_address as usize;
                    cycle -= 6;
                },
                Self::JMP_ABSOLUTE => {
                    let jmp_address = self.fetch_word(memory);
                    let fist_byte = memory.data[jmp_address as usize];
                    let second_byte = memory.data[(jmp_address + 1) as usize];
                    self.pc = ((second_byte as usize) << 8) | fist_byte as usize;
                    cycle -= 3;
                },
                _ => {
                    println!("Errore, istruzione {} non riconosciuta", instruction);
                    break;
                }
            }
        }
    }
}