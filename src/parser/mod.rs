use chumsky::prelude::*;
use real::instructions::Instruction;

use crate::parser::line::Line;

mod common;
mod line;
mod pseudo;
pub mod real;

pub fn program<'src>() -> impl Parser<'src, &'src str, Vec<Instruction>> {
    let real_ins = real::real_instructions();
    let pseudo_ins = pseudo::pseudo_instructions();

    real_ins.padded().repeated().collect()
}

pub fn pre_parser<'src>() -> impl Parser<'src, &'src str, Vec<Instruction>> {
    let real_ins = real::real_instructions().map(|r| Line::Instruction(r));
    let pseudo_ins = pseudo::pseudo_instructions().map(|p| Line::Pseudo(p));
    let lines = choice((real_ins, pseudo_ins));

    empty()
        .map(|_| vec![])
        .foldl(lines.padded().repeated(), |mut acc, l| {
            acc.extend(match l {
                Line::Instruction(r) => vec![r],
                Line::Pseudo(p) => p.expand(),
            });
            acc
        })
}
