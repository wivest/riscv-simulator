pub use super::names::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction<I, O> {
    BType {
        name: BType,
        rs1: u32,
        rs2: u32,
        offset: O,
    },
    IType {
        name: IType,
        rd: u32,
        rs: u32,
        imm: I,
    },
    JType {
        name: JType,
        rd: u32,
        imm: O,
    },
    RType {
        name: RType,
        rd: u32,
        rs1: u32,
        rs2: u32,
    },
    SType {
        name: SType,
        rs1: u32,
        rs2: u32,
        imm: I,
    },
    UType {
        name: UType,
        rd: u32,
        imm: I,
    },
    System(System),
}

impl std::fmt::Display for Instruction<i32, i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::BType {
                name,
                rs1,
                rs2,
                offset,
            } => write!(f, "{name} x{rs1}, x{rs2}, {offset}"),
            Self::IType { name, rd, rs, imm } => match name {
                IType::Jalr | IType::Lb | IType::Lbu | IType::Lh | IType::Lhu | IType::Lw => {
                    write!(f, "{name} x{rd}, {imm}(x{rs})")
                }
                _ => write!(f, "{name} x{rd}, x{rs}, {imm}"),
            },
            Self::JType { name, rd, imm } => write!(f, "{name} x{rd}, {imm}"),
            Self::RType { name, rd, rs1, rs2 } => write!(f, "{name} x{rd}, x{rs1}, x{rs2}"),
            Self::SType {
                name,
                rs1,
                rs2,
                imm,
            } => write!(f, "{name} x{rs2}, {imm}(x{rs1})"),
            Self::UType { name, rd, imm } => write!(f, "{name} x{rd}, {imm}"),
            Self::System(name) => write!(f, "{name}"),
        }
    }
}
