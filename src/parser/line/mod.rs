use crate::parser::pseudo::Pseudo;
use crate::parser::real::instructions::Instruction;

pub enum Line {
    Instruction(Instruction),
    Pseudo(Pseudo),
}
