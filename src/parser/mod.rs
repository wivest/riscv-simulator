use crate::language::{
    directive::{Directive, SectionName},
    instruction::*,
    token::{Definition, Immediate, Offset},
};

use chumsky::prelude::*;
use real::real_instructions;
use section::Section;

mod common;
mod directive;
pub mod section;
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

pub struct Program<'src> {
    pub text: Section<'src, Immediate<'src>, Offset<'src>>,
    pub data: Section<'src, Immediate<'src>, Offset<'src>>,
    pub rodata: Section<'src, Immediate<'src>, Offset<'src>>,
    pub bss: Section<'src, Immediate<'src>, Offset<'src>>,
}

impl<'src> Program<'src> {
    fn new() -> Self {
        Program {
            text: Section::new(0, 0),
            data: Section::new(0, 0),
            rodata: Section::new(0, 0),
            bss: Section::new(0, 0),
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

    line.padded()
        .recover_with(skip_then_retry_until(any().ignored(), just('\n').ignored()))
        .repeated()
        .collect::<Vec<_>>()
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

            process_line(line, curr, &mut active);
        }

        program
    })
}

fn process_line<'src>(
    line: Line<'src>,
    curr: &mut Section<'src, Immediate<'src>, Offset<'src>>,
    active: &mut SectionName,
) {
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
            curr.defs.insert(def, curr.pc);
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
        Line::Directive(Directive::Section(section)) => *active = section,
        Line::Directive(Directive::Ignore) | Line::Empty => {}
    }
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
