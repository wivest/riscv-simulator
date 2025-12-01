#[derive(Debug)]
pub enum Pseudo {
    Li { rd: usize, imm: i32 },
}

impl Pseudo {
    pub fn expand(self) -> String {
        match self {
            Self::Li { rd, imm } => format!(
                "lui x{}, {}\naddi x{}, x0, {}",
                rd,
                imm >> 12,
                rd,
                imm << 20 >> 20
            ),
        }
    }
}
