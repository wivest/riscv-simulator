use chumsky::prelude::*;

use instruction::Instruction;
use instruction::{BType, IType, JType, RType, SType, UType};

pub mod instruction;

fn register<'src>() -> impl Parser<'src, &'src str, usize> {
    let index = just("x")
        .ignore_then(text::int(10))
        .map(|s: &'src str| s.parse::<usize>().unwrap())
        .filter(|n| *n <= 31);

    let zero = just("zero").map(|_| 0);
    let ra = just("ra").map(|_| 1);
    let sp = just("sp").map(|_| 2);
    let gp = just("gp").map(|_| 3);
    let tp = just("tp").map(|_| 4);
    let fp = just("fp").map(|_| 8);

    let temporary = just("t")
        .ignore_then(text::int(10))
        .map(|s: &'src str| s.parse::<usize>().unwrap())
        .filter(|n| *n <= 6)
        .map(|n| if n <= 2 { n + 5 } else { n + 25 });

    let saved = just("s")
        .ignore_then(text::int(10))
        .map(|s: &'src str| s.parse::<usize>().unwrap())
        .filter(|n| *n <= 11)
        .map(|n| if n <= 1 { n + 8 } else { n + 16 });

    let argument = just("a")
        .ignore_then(text::int(10))
        .map(|s: &'src str| s.parse::<usize>().unwrap())
        .filter(|n| *n <= 7)
        .map(|n| n + 10);

    choice((index, zero, ra, sp, gp, tp, fp, temporary, saved, argument)).padded()
}

fn immediate<'src>() -> impl Parser<'src, &'src str, i32> {
    let sign = just("-").map(|_| -1).or(empty().map(|_| 1));
    let number = text::int(10).map(|s: &'src str| s.parse::<i32>().unwrap());
    sign.then_ignore(text::whitespace())
        .then(number)
        .map(|(s, n)| s * n)
        .padded()
}

fn immediate_bits<'src>(bits: u32) -> impl Parser<'src, &'src str, i32> {
    let sign = just("-").map(|_| -1).or(empty().map(|_| 1));
    let number = text::int(10).map(|s: &'src str| s.parse::<i32>().unwrap());
    sign.then_ignore(text::whitespace())
        .then(number)
        .filter(move |(_, n)| 0u32.leading_zeros() - n.leading_zeros() <= bits)
        .map(|(s, n)| s * n)
        .padded()
}

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
                .then(immediate()),
        )
        .map(move |([rd, rs], imm)| Instruction::IType { name, rd, rs, imm })
}

fn itype_instructions<'src>() -> impl Parser<'src, &'src str, Instruction> {
    let addi = itype(IType::Addi, just("addi"));
    let lb = itype_load(IType::Lb, just("lb"));

    choice((addi, lb))
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
                .then(immediate()),
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

    choice((beq,))
}

fn stype<'src>(
    name: SType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction> {
    prefix
        .ignore_then(
            register()
                .then_ignore(just(",").padded())
                .then(immediate())
                .then_ignore(just("(").padded())
                .then(register())
                .then_ignore(just(")").padded()),
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

    choice((sb,))
}

fn itype_load<'src>(
    name: IType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction> {
    prefix
        .ignore_then(
            register()
                .then_ignore(just(",").padded())
                .then(immediate())
                .then_ignore(just("(").padded())
                .then(register())
                .then_ignore(just(")").padded()),
        )
        .map(move |((rd, imm), rs)| Instruction::IType { name, rd, rs, imm })
}

fn jtype<'src>(
    name: JType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction> {
    prefix
        .ignore_then(register().then_ignore(just(",").padded()).then(immediate()))
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
        .ignore_then(register().then_ignore(just(",").padded()).then(immediate()))
        .map(move |(rd, imm)| Instruction::UType { name, rd, imm })
}

fn utype_instructions<'src>() -> impl Parser<'src, &'src str, Instruction> {
    let li = utype(UType::Li, just("li"));
    let lui = utype(UType::Lui, just("lui"));
    let auipc = utype(UType::Auipc, just("auipc"));

    choice((li, lui, auipc))
}

pub fn program<'src>() -> impl Parser<'src, &'src str, Vec<Instruction>> {
    let rtype_ins = rtype_instructions();
    let itype_ins = itype_instructions();
    let btype_ins = btype_instructions();
    let stype_ins = stype_instructions();
    let jtype_ins = jtype_instructions();
    let utype_ins = utype_instructions();

    let instruction = choice((
        rtype_ins, itype_ins, btype_ins, stype_ins, jtype_ins, utype_ins,
    ));

    instruction.padded().repeated().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_x() {
        let result = register().parse("x10");
        assert_eq!(result.unwrap(), 10);
    }

    #[test]
    fn test_register_name() {
        let zero = register().parse("zero");
        let ra = register().parse("ra");
        let sp = register().parse("sp");
        let gp = register().parse("gp");
        let tp = register().parse("tp");
        let fp = register().parse("fp");

        assert_eq!(zero.unwrap(), 0);
        assert_eq!(ra.unwrap(), 1);
        assert_eq!(sp.unwrap(), 2);
        assert_eq!(gp.unwrap(), 3);
        assert_eq!(tp.unwrap(), 4);
        assert_eq!(fp.unwrap(), 8);
    }

    #[test]
    fn test_register_name_index() {
        let t6 = register().parse("t6");
        let a7 = register().parse("a7");
        let s11 = register().parse("s11");

        assert_eq!(t6.unwrap(), 31);
        assert_eq!(a7.unwrap(), 17);
        assert_eq!(s11.unwrap(), 27);

        let t7 = register().parse("t7");
        let a8 = register().parse("a8");
        let s12 = register().parse("s12");

        assert_eq!(t7.has_errors(), true);
        assert_eq!(a8.has_errors(), true);
        assert_eq!(s12.has_errors(), true);
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
