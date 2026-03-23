use chumsky::prelude::*;
use label::Definition;
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
    Label(Definition),
}

pub fn program<'src>() -> impl Parser<'src, &'src str, (Vec<Instruction>, Vec<Definition>)> {
    let real_ins = real::real_instructions().map(|r| Line::Instruction(r));
    let pseudo_ins = pseudo::pseudo_instructions().map(|p| Line::Pseudo(p));
    let labels = label::label_def().map(|l| Line::Label(l));
    let line = choice((real_ins, pseudo_ins, labels));

    line.padded().repeated().collect::<Vec<_>>().map(|lines| {
        let mut instrs = Vec::new();
        let mut defs = Vec::new();

        for line in lines {
            match line {
                Line::Instruction(real) => instrs.push(real),
                Line::Pseudo(pseudo) => instrs.extend(pseudo),
                Line::Label(def) => defs.push(def),
            }
        }

        (instrs, defs)
    })
}
