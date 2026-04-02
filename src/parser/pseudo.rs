use super::token::{Immediate, Offset, label_ref, register};
use crate::instruction::*;
use crate::parser::common::*;

pub fn li<'src>() -> impl Parser<'src, &'src str, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("li")
        .ignore_then(register())
        .then_ignore(just(","))
        .then(number(32))
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

pub fn la<'src>() -> impl Parser<'src, &'src str, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("la")
        .ignore_then(register())
        .then_ignore(just(","))
        .then(label_ref())
        .map(move |(rd, label)| {
            vec![
                Instruction::UType {
                    name: UType::Lui,
                    rd,
                    imm: Immediate::Upper(label),
                },
                Instruction::IType {
                    name: IType::Addi,
                    rd,
                    rs: rd,
                    imm: Immediate::Lower(label),
                },
            ]
        })
}

pub fn pseudo_instructions<'src>()
-> impl Parser<'src, &'src str, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    choice((li(), la()))
}
