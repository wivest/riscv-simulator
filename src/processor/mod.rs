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

    pub fn store_data(&mut self, data: Vec<(usize, Vec<u8>)>) {
        for (at, bytes) in data {
            for (i, b) in bytes.into_iter().enumerate() {
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
