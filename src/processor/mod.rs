use crate::instruction::Instruction;
use memory::Memory;

pub mod instructions;
pub mod memory;

pub struct Processor {
    pub pc: usize,
    pub memory: Memory,
    registers: [i32; 32],
}

impl Processor {
    pub fn new(reset: usize) -> Self {
        Processor {
            pc: reset,
            registers: [0; 32],
            memory: Memory::new(),
        }
    }

    pub fn get_reg(&self, index: usize) -> i32 {
        if index == 0 { 0 } else { self.registers[index] }
    }

    pub fn set_reg(&mut self, index: usize, value: i32) {
        if index != 0 {
            self.registers[index] = value;
        };
    }

    pub fn store_strings(&mut self, strings: Vec<(usize, String)>) {
        for (at, string) in strings {
            for (i, byte) in string.bytes().enumerate() {
                self.memory.set(at + i, byte);
            }
            self.memory.set(at + string.len(), 0);
        }
    }

    pub fn store_bytes(&mut self, bytes: Vec<(usize, u8)>) {
        for (at, byte) in bytes {
            self.memory.set(at, byte);
        }
    }

    pub fn store_bytes2(&mut self, bytes: Vec<(usize, u16)>) {
        for (at, byte) in bytes {
            // TODO: endianness
            for (i, b) in byte.to_ne_bytes().into_iter().enumerate() {
                self.memory.set(at + i, b);
            }
        }
    }

    pub fn store_bytes4(&mut self, bytes: Vec<(usize, u32)>) {
        for (at, byte) in bytes {
            // TODO: endianness
            for (i, b) in byte.to_ne_bytes().into_iter().enumerate() {
                self.memory.set(at + i, b);
            }
        }
    }

    pub fn store_bytes8(&mut self, bytes: Vec<(usize, u64)>) {
        for (at, byte) in bytes {
            // TODO: endianness
            for (i, b) in byte.to_ne_bytes().into_iter().enumerate() {
                self.memory.set(at + i, b);
            }
        }
    }

    pub fn store_instrs(&mut self, instrs: Vec<(usize, Instruction<i32, i32>)>) {
        instrs.into_iter().for_each(|(addr, instr)| {
            self.memory.store_instr(addr, instr);
        });
    }

    pub fn execute(&mut self) {
        loop {
            let Some(instr) = self.memory.load_instr(self.pc) else {
                break;
            };
            println!("{instr:?}");
            if !instr.execute(self) {
                break;
            }
        }
    }
}
