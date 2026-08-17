use crate::language::{
    directive::{Directive, SectionName},
    instruction::*,
    token::{Definition, Immediate, Offset},
    word::Word,
};

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

type E<'src> = extra::Err<Rich<'src, char>>;
pub trait StrParser<'src, O>: Parser<'src, &'src str, O, E<'src>> {}
impl<'src, O, P> StrParser<'src, O> for P where P: Parser<'src, &'src str, O, E<'src>> {}

pub enum Line<'a> {
    Instruction(Instruction<Immediate<'a>, Offset<'a>>),
    Pseudo(Vec<Instruction<Immediate<'a>, Offset<'a>>>),
    Label(Definition<'a>),
    Directive(Directive),
    Empty,
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
            .insert((self.base + addr) / 4, Word::Instruction(instr));
    }
}

pub struct Program<'src> {
    pub defs: HashMap<Definition<'src>, usize>,
    pub text: Section<Immediate<'src>, Offset<'src>>,
    pub data: Section<Immediate<'src>, Offset<'src>>,
    pub rodata: Section<Immediate<'src>, Offset<'src>>,
    pub bss: Section<Immediate<'src>, Offset<'src>>,
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

fn lines<'src>() -> impl StrParser<'src, Vec<Line<'src>>> {
    let real_ins = real_instructions().map(|r| Line::Instruction(r));
    let pseudo_ins = pseudo::pseudo_instructions().map(|p| Line::Pseudo(p));
    let labels = token::label_def().map(|l| Line::Label(l));
    let dirs = directive::dirs().map(|d| Line::Directive(d));
    let comments = common::comment().map(|_| Line::Empty);
    let line = choice((real_ins, pseudo_ins, labels, dirs, comments));

    line.padded().repeated().collect::<Vec<_>>()
}

pub fn program<'src>() -> impl StrParser<'src, Program<'src>> {
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
                Line::Empty => {}
            }
        }

        program
    })
}

macro_rules! instructions {
    ($func:expr, $en:ident, [ $($var:ident),+ $(,)?]) => {
        choice(($({
            let name: &'static str = stringify!($var).to_lowercase().leak();
            $func($en::$var, just(name))
        },)+)).boxed()
    };
}

fn real_instructions<'src>() -> impl StrParser<'src, Instruction<Immediate<'src>, Offset<'src>>> {
    let btype_ins = instructions!(btype, BType, [Beq, Bne, Blt, Bltu, Bge, Bgeu]);
    let itype_ins = instructions!(
        itype,
        IType,
        [Addi, Andi, Ori, Xori, Slli, Srli, Srai, Jalr]
    );
    let iltype_ins = instructions!(itype_load, IType, [Lw, Lh, Lhu, Lb, Lbu]);
    let jtype_ins = instructions!(jtype, JType, [Jal]);
    let rtype_ins = instructions!(
        rtype,
        RType,
        [
            Add, Sub, Mul, Mulh, Mulhu, Mulhsu, Div, Rem, And, Or, Xor, Sll, Srl, Sra,
        ]
    );
    let stype_ins = instructions!(stype, SType, [Sw, Sh, Sb]);
    let utype_ins = instructions!(utype, UType, [Lui, Auipc]);
    let system_ins = system();

    choice((
        btype_ins, itype_ins, iltype_ins, jtype_ins, rtype_ins, stype_ins, utype_ins, system_ins,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtype() {
        let result = real_instructions().parse("add x0, x1, x2");
        assert_eq!(
            result.unwrap(),
            Instruction::RType {
                name: RType::Add,
                rd: 0,
                rs1: 1,
                rs2: 2
            }
        );
        let result = real_instructions().parse("add x0,\nx1, x2");
        assert_eq!(result.has_errors(), true);
    }
}
