use std::collections::HashMap;

use crate::parser::instruction::Instruction;

pub struct Processor {
    pub pc: usize,
    pub registers: [i32; 33],
    pub memory: HashMap<usize, u8>,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            pc: 0,
            registers: [0; 33],
            memory: HashMap::new(),
        }
    }

    pub fn execute(&mut self, instructions: Vec<Instruction>) {
        while self.pc / 4 < instructions.len() {
            let instruction = instructions.get(self.pc / 4).unwrap();
            println!("{instruction:?}");
            instruction.execute(self);
        }
    }
}
