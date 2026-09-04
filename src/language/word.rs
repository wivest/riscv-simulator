use super::instruction::Instruction;

#[derive(Debug)]
pub enum Word<I, O> {
    Instruction(Instruction<I, O>),
    Value(u32),
}

impl Word<i32, i32> {
    pub fn encode(&self) -> String {
        let split = |word: u32| word.to_be_bytes().map(|b| format!("{b:02x}")).join(" ");
        match *self {
            Word::Instruction(instr) => split(instr.encode()),
            Word::Value(value) => split(value),
        }
    }
}

impl std::fmt::Display for Word<i32, i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let split = |word: u32| word.to_be_bytes().map(|b| format!("{b:02x}")).join(" ");
        match *self {
            Self::Instruction(instr) => write!(f, "{}\t{instr}", split(instr.encode())),
            Self::Value(value) => write!(f, "{}", split(value)),
        }
    }
}
