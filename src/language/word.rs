use super::instruction::Instruction;

#[derive(Debug)]
pub enum Word<I, O> {
    Instruction(Instruction<I, O>),
    Value(u32),
}

impl std::fmt::Display for Word<i32, i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Instruction(instr) => write!(f, "{instr}"),
            Self::Value(value) => write!(f, "{value:08x}"),
        }
    }
}
