use std::collections::HashMap;

use crate::parser::Instruction;

pub struct Processor {
    pub pc: usize,
    pub registers: [i32; 32],
    pub memory: HashMap<i32, u8>,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            pc: 0,
            registers: [0; 32],
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
