use chumsky::prelude::*;
use label::Label;
use real::instructions::Instruction;

mod common;
pub mod immediate;
mod label;
mod pseudo;
pub mod real;
mod register;

pub enum Line {
    Instruction(Instruction),
    Pseudo(Vec<Instruction>),
    Label(Label),
}

pub fn program<'src>() -> impl Parser<'src, &'src str, Vec<Instruction>> {
    let real_ins = real::real_instructions().map(|r| Line::Instruction(r));
    let pseudo_ins = pseudo::pseudo_instructions().map(|p| Line::Pseudo(p));
    let lines = choice((real_ins, pseudo_ins));

    empty()
        .map(|_| vec![])
        .foldl(lines.padded().repeated(), |mut acc, l| {
            acc.extend(match l {
                Line::Instruction(r) => vec![r],
                Line::Pseudo(p) => p,
                Line::Label(_) => todo!(),
            });
            acc
        })
}
