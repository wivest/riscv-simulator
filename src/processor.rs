use crate::chumsky_parser::Instruction;

pub fn execute(instructions: Vec<Instruction>) {
    for instruction in instructions {
        println!("{instruction:?}");
        instruction.execute();
    }
}
