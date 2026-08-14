use crate::directive::{Directive, SectionName};
use crate::processor::memory::Word;
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

#[derive(Debug)]
pub struct Section<I, O> {
    pub base: usize,
    pub pc: usize,
    pub content: HashMap<usize, Word<I, O>>,
}

impl<I: Copy, O: Copy> Section<I, O> {
    pub fn new(base: usize) -> Self {
        Section {
            base,
            pc: 0,
            content: HashMap::new(),
        }
    }

    pub fn set(&mut self, addr: usize, value: u8) {
        let addr = self.base + addr;
        let cell = self.content.get(&(addr / 4)).unwrap_or(&Word::Value(0));
        let word = match cell {
            Word::Instruction(_) => return, // TODO: error
            Word::Value(v) => *v,
        };
        let mut bytes = word.to_ne_bytes();
        bytes[addr % 4] = value;
        self.content
            .insert(addr / 4, Word::Value(u32::from_ne_bytes(bytes)));
    }

    pub fn store_instr(&mut self, addr: usize, instr: Instruction<I, O>) {
        self.content
            .insert(self.base + addr / 4, Word::Instruction(instr));
    }
}

pub struct Program<'src> {
    pub defs: HashMap<token::Definition<'src>, usize>,
    pub text: Section<token::Immediate<'src>, token::Offset<'src>>,
    pub data: Section<token::Immediate<'src>, token::Offset<'src>>,
    pub rodata: Section<token::Immediate<'src>, token::Offset<'src>>,
    pub bss: Section<token::Immediate<'src>, token::Offset<'src>>,
}

impl<'src> Program<'src> {
    fn new() -> Self {
        Program {
            defs: HashMap::new(),
            text: Section::new(0),
            data: Section::new(0),
            rodata: Section::new(0),
            bss: Section::new(0),
        }
    }
}

fn lines<'src>() -> impl Parser<'src, &'src str, Vec<Line<'src>>> {
    let real_ins = real_instructions().map(|r| Line::Instruction(r));
    let pseudo_ins = pseudo::pseudo_instructions().map(|p| Line::Pseudo(p));
    let labels = token::label_def().map(|l| Line::Label(l));
    let dirs = directive::dirs().map(|d| Line::Directive(d));
    let line = choice((real_ins, pseudo_ins, labels, dirs));

    line.padded().repeated().collect::<Vec<_>>()
}

pub fn program<'src>() -> impl Parser<'src, &'src str, Program<'src>> {
    lines().map(|lines| {
        let mut program = Program::new();
        let mut active = SectionName::Text;

        for line in lines {
            let curr = match active {
                SectionName::Text => &mut program.text,
                SectionName::Data => &mut program.data,
                SectionName::Rodata => &mut program.rodata,
                SectionName::Bss => &mut program.bss,
            };

            match line {
                Line::Instruction(real) => {
                    curr.store_instr(curr.pc, real);
                    curr.pc += 4;
                }
                Line::Pseudo(pseudo) => {
                    for i in pseudo {
                        curr.store_instr(curr.pc, i);
                        curr.pc += 4;
                    }
                }
                Line::Label(def) => {
                    program.defs.insert(def, curr.pc);
                }
                Line::Directive(Directive::Org(at)) => curr.pc = at,
                Line::Directive(Directive::Unaligned(bytes)) => {
                    for b in bytes {
                        curr.set(curr.pc, b);
                        curr.pc += 1;
                    }
                }
                Line::Directive(Directive::Aligned(size, bytes)) => {
                    curr.pc = curr.pc.next_multiple_of(size);
                    for b in bytes {
                        curr.set(curr.pc, b);
                        curr.pc += 1;
                    }
                }
                Line::Directive(Directive::Section(section)) => active = section,
            }
        }

        program
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
