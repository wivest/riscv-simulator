use super::common::*;
use super::real::instructions::Instruction;
use crate::instructions::*;
use chumsky::prelude::*;

pub fn li<'src>() -> impl Parser<'src, &'src str, Vec<Instruction>> {
    just("li")
        .ignore_then(register().then_ignore(just(",")).then(immediate(32)))
        .map(move |(rd, imm)| {
            vec![
                Instruction::UType {
                    name: UType::Lui,
                    rd,
                    imm: imm >> 12,
                },
                Instruction::IType {
                    name: IType::Addi,
                    rd,
                    rs: 0,
                    imm: imm << 20 >> 20,
                },
            ]
        })
}

pub fn pseudo_instructions<'src>() -> impl Parser<'src, &'src str, Vec<Instruction>> {
    choice((li(),))
}
