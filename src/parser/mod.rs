use chumsky::prelude::*;
use real::instructions::Instruction;

mod common;
mod line;
mod pseudo;
pub mod real;

pub fn program<'src>() -> impl Parser<'src, &'src str, Vec<Instruction>> {
    let real_ins = real::real_instructions();
    let pseudo_ins = pseudo::pseudo_instructions();

    real_ins.padded().repeated().collect()
}

pub fn pre_parse<'src>() -> impl Parser<'src, &'src str, ()> {
    empty()
}
