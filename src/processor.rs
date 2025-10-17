use crate::chumsky_parser::Instruction;

pub struct Processor {
    pub registers: [i32; 32],
}

impl Processor {
    pub fn new() -> Self {
        Processor { registers: [0; 32] }
    }

    pub fn execute(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            println!("{instruction:?}");
            instruction.execute(self);
        }
    }
}
