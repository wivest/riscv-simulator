use instructions::Instruction;
use memory::Memory;

pub mod instructions;
pub mod memory;

pub struct Processor {
    pub pc: usize,
    pub memory: Memory,
    registers: [i32; 32],
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            pc: 0,
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

    pub fn execute(&mut self, instructions: Vec<Instruction>) {
        while self.pc / 4 < instructions.len() {
            let instruction = instructions.get(self.pc / 4).unwrap();
            println!("{instruction:?}");
            instruction.execute(self);
        }
    }
}
