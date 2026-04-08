use super::token::{Immediate, Offset, immediate12, immediate20, offset, register};
pub use crate::instruction::*;
use crate::parser::common::*;

pub fn btype<'src>(
    name: BType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction<Immediate<'src>, Offset<'src>>> {
    prefix
        .ignore_then(register().then_arg(register()).then_arg(offset(13)))
        .map(move |((rs1, rs2), offset)| Instruction::BType {
            name,
            rs1,
            rs2,
            offset,
        })
}

pub fn itype<'src>(
    name: IType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction<Immediate<'src>, Offset<'src>>> {
    prefix
        .ignore_then(register().then_arg(register()).then_arg(immediate12()))
        .map(move |((rd, rs), imm)| Instruction::IType { name, rd, rs, imm })
}

pub fn itype_load<'src>(
    name: IType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction<Immediate<'src>, Offset<'src>>> {
    let load = immediate12().index(register());
    prefix
        .ignore_then(register().then_arg(load))
        .map(move |(rd, (imm, rs))| Instruction::IType { name, rd, rs, imm })
}

pub fn jtype<'src>(
    name: JType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction<Immediate<'src>, Offset<'src>>> {
    prefix
        .ignore_then(register().then_arg(offset(21)))
        .map(move |(rd, imm)| Instruction::JType { name, rd, imm })
}

pub fn rtype<'src>(
    name: RType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction<Immediate<'src>, Offset<'src>>> {
    prefix
        .ignore_then(register().then_arg(register()).then_arg(register()))
        .map(move |((rd, rs1), rs2)| Instruction::RType { name, rd, rs1, rs2 })
}

pub fn stype<'src>(
    name: SType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction<Immediate<'src>, Offset<'src>>> {
    let store = immediate12().index(register());
    prefix
        .ignore_then(register().then_arg(store))
        .map(move |(rs2, (imm, rs1))| Instruction::SType {
            name,
            rs1,
            rs2,
            imm,
        })
}

pub fn utype<'src>(
    name: UType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction<Immediate<'src>, Offset<'src>>> {
    prefix
        .ignore_then(register().then_arg(immediate20()))
        .map(move |(rd, imm)| Instruction::UType { name, rd, imm })
}

pub fn system<'src>() -> impl Parser<'src, &'src str, Instruction<Immediate<'src>, Offset<'src>>> {
    let ebreak = just("ebreak").map(|_| Instruction::System(System::Ebreak));

    choice((ebreak,))
}
