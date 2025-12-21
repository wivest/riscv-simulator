use super::label::Label;
use super::pseudo::Pseudo;
use super::real::instructions::Instruction;

pub enum Line {
    Instruction(Instruction),
    Pseudo(Pseudo),
    Label(Label),
}
