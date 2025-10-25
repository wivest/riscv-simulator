use chumsky::prelude::*;

use instruction::Instruction;
use instruction::{BType, IType, RType, SType};

pub mod instruction;

fn register<'src>() -> impl Parser<'src, &'src str, i32> {
    just("x")
        .ignore_then(text::int(10))
        .map(|s: &'src str| s.parse::<i32>().unwrap())
}

fn immediate<'src>() -> impl Parser<'src, &'src str, i32> {
    let sign = just("-").map(|_| -1).or(empty().map(|_| 1));
    let number = text::int(10).map(|s: &'src str| s.parse::<i32>().unwrap());
    sign.then_ignore(text::whitespace())
        .then(number)
        .map(|(s, n)| s * n)
}

fn rtype<'src>(
    name: RType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction> {
    prefix
        .ignore_then(
            register()
                .padded()
                .separated_by(just(","))
                .collect_exactly::<[_; 3]>(),
        )
        .map(move |[rd, rs1, rs2]| Instruction::RType { name, rd, rs1, rs2 })
}

fn itype<'src>(
    name: IType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction> {
    prefix
        .ignore_then(
            register()
                .padded()
                .separated_by(just(","))
                .collect_exactly::<[_; 2]>()
                .then_ignore(just(","))
                .then(immediate().padded()),
        )
        .map(move |([rd, rs], imm)| Instruction::IType { name, rd, rs, imm })
}

fn btype<'src>(
    name: BType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction> {
    prefix
        .ignore_then(
            register()
                .padded()
                .separated_by(just(","))
                .collect_exactly::<[_; 2]>()
                .then_ignore(just(","))
                .then(immediate().padded()),
        )
        .map(move |([rs1, rs2], offset)| Instruction::BType {
            name,
            rs1,
            rs2,
            offset,
        })
}

fn stype<'src>(
    name: SType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction> {
    prefix
        .ignore_then(
            register()
                .padded()
                .then_ignore(just(",").padded())
                .then(immediate().padded())
                .then_ignore(just("(").padded())
                .then(register().padded())
                .then_ignore(just(")").padded()),
        )
        .map(move |((rs2, imm), rs1)| Instruction::SType {
            name,
            rs1,
            rs2,
            imm,
        })
}

pub fn program<'src>() -> impl Parser<'src, &'src str, Vec<Instruction>> {
    let add = rtype(RType::Add, just("add"));
    let sub = rtype(RType::Sub, just("sub"));
    let mul = rtype(RType::Mul, just("mul"));
    let div = rtype(RType::Div, just("div"));
    let rem = rtype(RType::Rem, just("rem"));
    let addi = itype(IType::Addi, just("addi"));
    let beq = btype(BType::Beq, just("beq"));
    let sb = stype(SType::Sb, just("sb"));
    let instruction = choice((add, sub, mul, div, rem, addi, beq, sb));

    instruction.padded().repeated().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let result = register().parse("x10");
        assert_eq!(result.unwrap(), 10);
    }

    #[test]
    fn test_immediate() {
        let result = immediate().parse("42");
        assert_eq!(result.unwrap(), 42);
        let result = immediate().parse("-42");
        assert_eq!(result.unwrap(), -42);
        let result = immediate().parse("- 42");
        assert_eq!(result.unwrap(), -42);
    }
}
