use crate::parser::instruction::Instruction;
use crate::parser::pseudo::Pseudo;

pub enum Line {
    Instruction(Instruction),
    Pseudo(Pseudo),
}
