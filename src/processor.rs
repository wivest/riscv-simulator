use std::collections::HashMap;

use crate::parser::instruction::Instruction;

pub struct Processor {
    pub registers: [i32; 33],
    pub memory: HashMap<usize, u8>,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            registers: [0; 33],
            memory: HashMap::new(),
        }
    }

    pub fn get_pc(&self) -> usize {
        self.registers[32] as usize
    }

    pub fn set_pc(&mut self, value: usize) {
        self.registers[32] = value as i32;
    }

    pub fn execute(&mut self, instructions: Vec<Instruction>) {
        while self.get_pc() / 4 < instructions.len() {
            let instruction = instructions.get(self.get_pc() / 4).unwrap();
            println!("{instruction:?}");
            instruction.execute(self);
        }
    }
}
