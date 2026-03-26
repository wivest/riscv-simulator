use chumsky::prelude::*;
use label::Definition;
use real::*;
use std::collections::HashMap;

mod common;
pub mod grammar;
pub mod immediate;
pub mod label;
mod pseudo;
pub mod real;
mod register;

pub enum Line<'a> {
    Instruction(Instruction<'a>),
    Pseudo(Vec<Instruction<'a>>),
    Label(Definition<'a>),
}

pub fn program<'src>()
-> impl Parser<'src, &'src str, (Vec<Instruction<'src>>, HashMap<Definition<'src>, usize>)> {
    let real_ins = real_instructions().map(|r| Line::Instruction(r));
    let pseudo_ins = pseudo::pseudo_instructions().map(|p| Line::Pseudo(p));
    let labels = label::label_def().map(|l| Line::Label(l));
    let line = choice((real_ins, pseudo_ins, labels));

    line.padded().repeated().collect::<Vec<_>>().map(|lines| {
        let mut instrs = Vec::new();
        let mut defs = HashMap::new();

        for line in lines {
            match line {
                Line::Instruction(real) => instrs.push(real),
                Line::Pseudo(pseudo) => instrs.extend(pseudo),
                Line::Label(def) => {
                    defs.insert(def, instrs.len());
                    ()
                }
            }
        }

        (instrs, defs)
    })
}

fn btype_instructions<'src>() -> impl Parser<'src, &'src str, Instruction<'src>> {
    let beq = btype(BType::Beq, just("beq"));
    let bne = btype(BType::Bne, just("bne"));

    choice((beq, bne))
}

fn itype_instructions<'src>() -> impl Parser<'src, &'src str, Instruction<'src>> {
    let addi = itype(IType::Addi, just("addi"));
    let jalr = itype(IType::Jalr, just("jalr"));
    let lb = itype_load(IType::Lb, just("lb"));
    let lh = itype_load(IType::Lh, just("lh"));
    let lw = itype_load(IType::Lw, just("lw"));

    choice((addi, jalr, lb, lh, lw))
}

fn jtype_instructions<'src>() -> impl Parser<'src, &'src str, Instruction<'src>> {
    let jal = jtype(JType::Jal, just("jal"));

    choice((jal,))
}

fn rtype_instructions<'src>() -> impl Parser<'src, &'src str, Instruction<'src>> {
    let add = rtype(RType::Add, just("add"));
    let sub = rtype(RType::Sub, just("sub"));
    let mul = rtype(RType::Mul, just("mul"));
    let div = rtype(RType::Div, just("div"));
    let rem = rtype(RType::Rem, just("rem"));
    let and = rtype(RType::And, just("and"));
    let or = rtype(RType::Or, just("or"));
    let xor = rtype(RType::Xor, just("xor"));

    choice((add, sub, mul, div, rem, and, or, xor))
}

fn stype_instructions<'src>() -> impl Parser<'src, &'src str, Instruction<'src>> {
    let sb = stype(SType::Sb, just("sb"));
    let sh = stype(SType::Sh, just("sh"));
    let sw = stype(SType::Sw, just("sw"));

    choice((sb, sh, sw))
}

fn utype_instructions<'src>() -> impl Parser<'src, &'src str, Instruction<'src>> {
    let lui = utype(UType::Lui, just("lui"));
    let auipc = utype(UType::Auipc, just("auipc"));

    choice((lui, auipc))
}

fn real_instructions<'src>() -> impl Parser<'src, &'src str, Instruction<'src>> {
    let rtype_ins = rtype_instructions();
    let itype_ins = itype_instructions();
    let btype_ins = btype_instructions();
    let stype_ins = stype_instructions();
    let jtype_ins = jtype_instructions();
    let utype_ins = utype_instructions();
    let system_ins = system();

    choice((
        rtype_ins, itype_ins, btype_ins, stype_ins, jtype_ins, utype_ins, system_ins,
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
        let result = rtype_instructions().parse("add x0,\nx1, x2");
        assert_eq!(result.has_errors(), true);
    }
}
