use super::instruction::Instruction;

#[derive(Debug)]
pub enum Word<I, O> {
    Instruction(Instruction<I, O>),
    Value(u32),
}

impl Word<i32, i32> {
    pub fn encode(&self) -> u32 {
        match *self {
            Word::Instruction(instr) => instr.encode(),
            Word::Value(value) => value,
        }
    }

    pub fn split(&self) -> String {
        self.encode()
            .to_le_bytes()
            .map(|b| format!("{b:02x}"))
            .join(" ")
    }

    pub fn ascii(&self) -> String {
        let bytes = self.encode().to_le_bytes();
        let ascii = bytes.map(|b| match b.is_ascii_graphic() {
            true => b as char,
            false => '.',
        });
        String::from_iter(ascii)
    }
}

impl std::fmt::Display for Word<i32, i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Instruction(instr) => write!(f, "{}\t{instr}", self.split()),
            Self::Value(_) => write!(f, "{}\t{}", self.split(), self.ascii()),
        }
    }
}
