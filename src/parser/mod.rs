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

pub fn pre_parser<'src>() -> impl Parser<'src, &'src str, Vec<Line>> {
    let real_ins = real::real_instructions().map(|r| Line::Instruction(r));
    let pseudo_ins = pseudo::pseudo_instructions().map(|p| Line::Pseudo(p));
    let lines = choice((real_ins, pseudo_ins));

    lines.padded().repeated().collect()
}
