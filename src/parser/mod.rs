use crate::directive::Directive;
use chumsky::prelude::*;
use real::*;
use std::collections::HashMap;

mod common;
mod directive;
pub mod token {
    pub use immediate::*;
    pub use label::*;
    pub use register::*;

    mod immediate;
    mod label;
    mod register;
}
mod pseudo;
pub mod real;

pub enum Line<'a> {
    Instruction(Instruction<token::Immediate<'a>, token::Offset<'a>>),
    Pseudo(Vec<Instruction<token::Immediate<'a>, token::Offset<'a>>>),
    Label(token::Definition<'a>),
    Directive(Directive),
}

pub fn program<'src>() -> impl Parser<
    'src,
    &'src str,
    (
        Vec<(usize, String)>,
        Vec<(usize, u8)>,
        Vec<(usize, u16)>,
        Vec<(usize, u32)>,
        Vec<(usize, u64)>,
        Vec<(
            usize,
            Instruction<token::Immediate<'src>, token::Offset<'src>>,
        )>,
        HashMap<token::Definition<'src>, usize>,
    ),
> {
    let real_ins = real_instructions().map(|r| Line::Instruction(r));
    let pseudo_ins = pseudo::pseudo_instructions().map(|p| Line::Pseudo(p));
    let labels = token::label_def().map(|l| Line::Label(l));
    let dirs = directive::dirs().map(|d| Line::Directive(d));
    let line = choice((real_ins, pseudo_ins, labels, dirs));

    line.padded().repeated().collect::<Vec<_>>().map(|lines| {
        let mut pc = 0usize;
        let mut strings = Vec::new();
        let mut bytes = Vec::new();
        let mut bytes2 = Vec::new();
        let mut bytes4 = Vec::new();
        let mut bytes8 = Vec::new();
        let mut instrs = Vec::new();
        let mut defs = HashMap::new();

        for line in lines {
            match line {
                Line::Instruction(real) => instrs.push((
                    {
                        pc += 4;
                        pc - 4
                    },
                    real,
                )),
                Line::Pseudo(pseudo) => instrs.extend(pseudo.into_iter().map(|instr| {
                    (
                        {
                            pc += 4;
                            pc - 4
                        },
                        instr,
                    )
                })),
                Line::Label(def) => {
                    defs.insert(def, pc);
                    ()
                }
                Line::Directive(Directive::Org(at)) => pc = at,
                Line::Directive(Directive::Asciz(s)) => {
                    let slen = s.len() + 1;
                    strings.push((pc, s));
                    pc += slen;
                }
                Line::Directive(Directive::Byte(b)) => {
                    bytes.push((pc, b));
                    pc += 1;
                }
                Line::Directive(Directive::Byte2(b)) => {
                    bytes2.push((pc, b));
                    pc += 2;
                }
                Line::Directive(Directive::Byte4(b)) => {
                    bytes4.push((pc, b));
                    pc += 4;
                }
                Line::Directive(Directive::Byte8(b)) => {
                    bytes8.push((pc, b));
                    pc += 8;
                }
            }
        }

        (strings, bytes, bytes2, bytes4, bytes8, instrs, defs)
    })
}

fn btype_instructions<'src>()
-> impl Parser<'src, &'src str, Instruction<token::Immediate<'src>, token::Offset<'src>>> {
    let beq = btype(BType::Beq, just("beq"));
    let bne = btype(BType::Bne, just("bne"));

    choice((beq, bne))
}

fn itype_instructions<'src>()
-> impl Parser<'src, &'src str, Instruction<token::Immediate<'src>, token::Offset<'src>>> {
    let addi = itype(IType::Addi, just("addi"));
    let jalr = itype(IType::Jalr, just("jalr"));
    let lb = itype_load(IType::Lb, just("lb"));
    let lh = itype_load(IType::Lh, just("lh"));
    let lw = itype_load(IType::Lw, just("lw"));

    choice((addi, jalr, lb, lh, lw))
}

fn jtype_instructions<'src>()
-> impl Parser<'src, &'src str, Instruction<token::Immediate<'src>, token::Offset<'src>>> {
    let jal = jtype(JType::Jal, just("jal"));

    choice((jal,))
}

fn rtype_instructions<'src>()
-> impl Parser<'src, &'src str, Instruction<token::Immediate<'src>, token::Offset<'src>>> {
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

fn stype_instructions<'src>()
-> impl Parser<'src, &'src str, Instruction<token::Immediate<'src>, token::Offset<'src>>> {
    let sb = stype(SType::Sb, just("sb"));
    let sh = stype(SType::Sh, just("sh"));
    let sw = stype(SType::Sw, just("sw"));

    choice((sb, sh, sw))
}

fn utype_instructions<'src>()
-> impl Parser<'src, &'src str, Instruction<token::Immediate<'src>, token::Offset<'src>>> {
    let lui = utype(UType::Lui, just("lui"));
    let auipc = utype(UType::Auipc, just("auipc"));

    choice((lui, auipc))
}

fn real_instructions<'src>()
-> impl Parser<'src, &'src str, Instruction<token::Immediate<'src>, token::Offset<'src>>> {
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
