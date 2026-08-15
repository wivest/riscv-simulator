use super::instruction::Instruction;

#[derive(Debug)]
pub enum Word<I, O> {
    Instruction(Instruction<I, O>),
    Value(u32),
}
