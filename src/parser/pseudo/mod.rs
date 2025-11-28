enum Pseudo {
    Li { rd: usize, imm: i32 },
}

impl Pseudo {
    fn expand(self) -> String {
        match self {
            Self::Li { rd, imm } => "todo".to_owned(),
        }
    }
}
