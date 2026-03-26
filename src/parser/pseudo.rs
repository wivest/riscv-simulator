use crate::parser::label::label_ref;

use super::grammar::*;
use super::immediate::*;
use super::register::*;
use chumsky::prelude::*;

pub fn li<'src>() -> impl Parser<'src, &'src str, Vec<Instruction<'src>>> {
    just("li")
        .ignore_then(register())
        .then_ignore(just(","))
        .then(immediate(32))
        .map(move |(rd, imm)| {
            vec![
                Instruction::UType {
                    name: UType::Lui,
                    rd,
                    imm: Immediate::Value(imm >> 12),
                },
                Instruction::IType {
                    name: IType::Addi,
                    rd,
                    rs: 0,
                    imm: Immediate::Value(imm << 20 >> 20), // TODO: test for negative edge case
                },
            ]
        })
}

pub fn la<'src>() -> impl Parser<'src, &'src str, Vec<Instruction<'src>>> {
    just("la")
        .ignore_then(register())
        .then_ignore(just(","))
        .then(label_ref())
        .map(move |(rd, label)| {
            vec![
                Instruction::UType {
                    name: UType::Lui,
                    rd,
                    imm: Immediate::Upper(label), // TODO: label address to be resolved at link time
                },
                Instruction::IType {
                    name: IType::Addi,
                    rd,
                    rs: rd,
                    imm: Immediate::Lower(label), // TODO: label address to be resolved at link time
                },
            ]
        })
}

pub fn pseudo_instructions<'src>() -> impl Parser<'src, &'src str, Vec<Instruction<'src>>> {
    choice((li(), la()))
}
