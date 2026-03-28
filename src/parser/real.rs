pub use super::grammar::*;
use super::token::{immediate12, immediate20, offset, register};
use crate::parser::common::*;

pub fn btype<'src>(
    name: BType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction<'src>> {
    prefix
        .ignore_then(
            register()
                .separated_by(just(","))
                .collect_exactly::<[_; 2]>()
                .then_ignore(just(","))
                .then(offset(13)),
        )
        .map(move |([rs1, rs2], offset)| Instruction::BType {
            name,
            rs1,
            rs2,
            offset,
        })
}

pub fn itype<'src>(
    name: IType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction<'src>> {
    prefix
        .ignore_then(
            register()
                .separated_by(just(","))
                .collect_exactly::<[_; 2]>()
                .then_ignore(just(","))
                .then(immediate12()),
        )
        .map(move |([rd, rs], imm)| Instruction::IType { name, rd, rs, imm })
}

pub fn itype_load<'src>(
    name: IType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction<'src>> {
    prefix
        .ignore_then(
            register()
                .then_ignore(just(","))
                .then(immediate12())
                .then_ignore(just("("))
                .then(register())
                .then_ignore(just(")")),
        )
        .map(move |((rd, imm), rs)| Instruction::IType { name, rd, rs, imm })
}

pub fn jtype<'src>(
    name: JType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction<'src>> {
    prefix
        .ignore_then(register().then_ignore(just(",")).then(offset(21)))
        .map(move |(rd, imm)| Instruction::JType { name, rd, imm })
}

pub fn rtype<'src>(
    name: RType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction<'src>> {
    prefix
        .ignore_then(
            register()
                .separated_by(just(","))
                .collect_exactly::<[_; 3]>(),
        )
        .map(move |[rd, rs1, rs2]| Instruction::RType { name, rd, rs1, rs2 })
}

pub fn stype<'src>(
    name: SType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction<'src>> {
    prefix
        .ignore_then(
            register()
                .then_ignore(just(","))
                .then(number(32)) // TODO: must be changed
                .then_ignore(just("("))
                .then(register())
                .then_ignore(just(")")),
        )
        .map(move |((rs2, imm), rs1)| Instruction::SType {
            name,
            rs1,
            rs2,
            imm,
        })
}

pub fn utype<'src>(
    name: UType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction<'src>> {
    prefix
        .ignore_then(register().then_ignore(just(",")).then(immediate20()))
        .map(move |(rd, imm)| Instruction::UType { name, rd, imm })
}

pub fn system<'src>() -> impl Parser<'src, &'src str, Instruction<'src>> {
    let ebreak = just("ebreak").map(|_| Instruction::System(System::Ebreak));

    choice((ebreak,))
}
