use crate::parser::real::instructions::{IType, Instruction, UType};

#[derive(Debug)]
pub enum Pseudo {
    Li { rd: usize, imm: i32 },
}

impl Pseudo {
    pub fn expand(self) -> Vec<Instruction> {
        match self {
            Self::Li { rd, imm } => vec![
                Instruction::UType {
                    name: UType::Lui,
                    rd,
                    imm: imm >> 12,
                },
                Instruction::IType {
                    name: IType::Addi,
                    rd,
                    rs: 0,
                    imm: imm << 20 >> 20,
                },
            ],
        }
    }
}
