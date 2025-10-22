use std::collections::HashMap;

use crate::chumsky_parser::Instruction;

pub struct Processor {
    pub pc: i32,
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
        for instruction in instructions {
            println!("{instruction:?}");
            instruction.execute(self);
        }
    }
}
