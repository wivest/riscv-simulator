use super::common::*;
use chumsky::prelude::*;
use instructions::Instruction;
use instructions::{BType, IType, JType, RType, SType, UType};

pub mod instructions;

fn rtype<'src>(
    name: RType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction> {
    prefix
        .ignore_then(
            register()
                .separated_by(just(","))
                .collect_exactly::<[_; 3]>(),
        )
        .map(move |[rd, rs1, rs2]| Instruction::RType { name, rd, rs1, rs2 })
}

fn rtype_instructions<'src>() -> impl Parser<'src, &'src str, Instruction> {
    let add = rtype(RType::Add, just("add"));
    let sub = rtype(RType::Sub, just("sub"));
    let mul = rtype(RType::Mul, just("mul"));
    let div = rtype(RType::Div, just("div"));
    let rem = rtype(RType::Rem, just("rem"));

    choice((add, sub, mul, div, rem))
}

fn itype<'src>(
    name: IType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction> {
    prefix
        .ignore_then(
            register()
                .separated_by(just(","))
                .collect_exactly::<[_; 2]>()
                .then_ignore(just(","))
                .then(immediate(12)),
        )
        .map(move |([rd, rs], imm)| Instruction::IType { name, rd, rs, imm })
}

fn itype_instructions<'src>() -> impl Parser<'src, &'src str, Instruction> {
    let addi = itype(IType::Addi, just("addi"));
    let lb = itype_load(IType::Lb, just("lb"));
    let lh = itype_load(IType::Lh, just("lh"));
    let lw = itype_load(IType::Lw, just("lw"));

    choice((addi, lb, lh, lw))
}

fn btype<'src>(
    name: BType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction> {
    prefix
        .ignore_then(
            register()
                .separated_by(just(","))
                .collect_exactly::<[_; 2]>()
                .then_ignore(just(","))
                .then(immediate(13)),
        )
        .map(move |([rs1, rs2], offset)| Instruction::BType {
            name,
            rs1,
            rs2,
            offset,
        })
}

fn btype_instructions<'src>() -> impl Parser<'src, &'src str, Instruction> {
    let beq = btype(BType::Beq, just("beq"));
    let bne = btype(BType::Bne, just("bne"));

    choice((beq, bne))
}

fn stype<'src>(
    name: SType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction> {
    prefix
        .ignore_then(
            register()
                .then_ignore(just(","))
                .then(immediate(32))
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

fn stype_instructions<'src>() -> impl Parser<'src, &'src str, Instruction> {
    let sb = stype(SType::Sb, just("sb"));
    let sh = stype(SType::Sh, just("sh"));
    let sw = stype(SType::Sw, just("sw"));

    choice((sb, sh, sw))
}

fn itype_load<'src>(
    name: IType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction> {
    prefix
        .ignore_then(
            register()
                .then_ignore(just(","))
                .then(immediate(32))
                .then_ignore(just("("))
                .then(register())
                .then_ignore(just(")")),
        )
        .map(move |((rd, imm), rs)| Instruction::IType { name, rd, rs, imm })
}

fn jtype<'src>(
    name: JType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction> {
    prefix
        .ignore_then(register().then_ignore(just(",")).then(immediate(21)))
        .map(move |(rd, imm)| Instruction::JType { name, rd, imm })
}

fn jtype_instructions<'src>() -> impl Parser<'src, &'src str, Instruction> {
    let jal = jtype(JType::Jal, just("jal"));

    choice((jal,))
}

fn utype<'src>(
    name: UType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction> {
    prefix
        .ignore_then(register().then_ignore(just(",")).then(immediate(20)))
        .map(move |(rd, imm)| Instruction::UType { name, rd, imm })
}

fn utype_instructions<'src>() -> impl Parser<'src, &'src str, Instruction> {
    let lui = utype(UType::Lui, just("lui"));
    let auipc = utype(UType::Auipc, just("auipc"));

    choice((lui, auipc))
}

pub fn real_instructions<'src>() -> impl Parser<'src, &'src str, Instruction> {
    let rtype_ins = rtype_instructions();
    let itype_ins = itype_instructions();
    let btype_ins = btype_instructions();
    let stype_ins = stype_instructions();
    let jtype_ins = jtype_instructions();
    let utype_ins = utype_instructions();

    choice((
        rtype_ins, itype_ins, btype_ins, stype_ins, jtype_ins, utype_ins,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtype() {
        let result = rtype_instructions().parse("add x0, x1, x2");
        assert_eq!(
            result.unwrap(),
            Instruction::RType {
                name: RType::Add,
                rd: 0,
                rs1: 1,
                rs2: 2
            }
        );
    }
}
