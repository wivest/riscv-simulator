use chumsky::prelude::*;

use instruction::Instruction;
use instruction::{BType, IType, JType, RType, SType, UType};

pub mod instruction;

fn number<'src, T: std::str::FromStr>() -> impl Parser<'src, &'src str, T> {
    text::int(10).map(|s: &'src str| s.parse::<T>().ok().unwrap())
}

fn register<'src>() -> impl Parser<'src, &'src str, usize> {
    let index = just("x").ignore_then(number()).filter(|n| *n <= 31);

    let zero = just("zero").map(|_| 0);
    let ra = just("ra").map(|_| 1);
    let sp = just("sp").map(|_| 2);
    let gp = just("gp").map(|_| 3);
    let tp = just("tp").map(|_| 4);
    let fp = just("fp").map(|_| 8);

    let temporary = just("t")
        .ignore_then(number::<usize>())
        .filter(|n| *n <= 6)
        .map(|n| if n <= 2 { n + 5 } else { n + 25 });

    let saved = just("s")
        .ignore_then(number::<usize>())
        .filter(|n| *n <= 11)
        .map(|n| if n <= 1 { n + 8 } else { n + 16 });

    let argument = just("a")
        .ignore_then(number::<usize>())
        .filter(|n| *n <= 7)
        .map(|n| n + 10);

    choice((index, zero, ra, sp, gp, tp, fp, temporary, saved, argument)).padded()
}

fn immediate<'src>(bits: u32) -> impl Parser<'src, &'src str, i32> {
    let sign = just("-").map(|_| -1).or(empty().map(|_| 1));
    sign.then_ignore(text::whitespace())
        .then(number::<i32>())
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
        let result = immediate(32).parse("42");
        assert_eq!(result.unwrap(), 42);
        let result = immediate(32).parse("-42");
        assert_eq!(result.unwrap(), -42);
        let result = immediate(32).parse("- 42");
        assert_eq!(result.unwrap(), -42);

        let result = immediate(12).parse("4095");
        assert_eq!(result.unwrap(), 4095);
        let result = immediate(12).parse("4096");
        assert_eq!(result.has_errors(), true);
    }

    #[test]
    fn test_rtype() {
        let result = rtype_instructions().parse("add x0, x1, x2");
        match result.unwrap() {
            Instruction::RType { name, rd, rs1, rs2 } => {
                match name {
                    RType::Add => {}
                    _ => panic!(),
                }
                assert_eq!(rd, 0);
                assert_eq!(rs1, 1);
                assert_eq!(rs2, 2);
            }
            _ => panic!(),
        }
    }
}
