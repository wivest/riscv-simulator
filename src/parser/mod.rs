use chumsky::prelude::*;
use real::instructions::Instruction;

mod common;
mod line;
mod pseudo;
pub mod real;

pub fn program<'src>() -> impl Parser<'src, &'src str, Vec<Instruction>> {
    real::program()
}

pub fn pre_parse<'src>() -> impl Parser<'src, &'src str, ()> {
    empty()
}
