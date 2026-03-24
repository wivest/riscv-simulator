use super::grammar::*;
use super::immediate::*;
use super::register::*;
use chumsky::prelude::*;

pub fn li<'src>() -> impl Parser<'src, &'src str, Vec<Instruction<'src>>> {
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

pub fn pseudo_instructions<'src>() -> impl Parser<'src, &'src str, Vec<Instruction<'src>>> {
    choice((li(),))
}
