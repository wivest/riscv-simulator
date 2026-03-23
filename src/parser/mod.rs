use chumsky::prelude::*;
use label::Definition;
use real::instructions::Instruction;
use std::collections::HashMap;

mod common;
pub mod immediate;
pub mod label;
mod pseudo;
pub mod real;
mod register;

pub enum Line<'a> {
    Instruction(Instruction<'a>),
    Pseudo(Vec<Instruction<'a>>),
    Label(Definition<'a>),
}

pub fn program<'src>()
-> impl Parser<'src, &'src str, (Vec<Instruction<'src>>, HashMap<Definition<'src>, usize>)> {
    let real_ins = real::real_instructions().map(|r| Line::Instruction(r));
    let pseudo_ins = pseudo::pseudo_instructions().map(|p| Line::Pseudo(p));
    let labels = label::label_def().map(|l| Line::Label(l));
    let line = choice((real_ins, pseudo_ins, labels));

    line.padded().repeated().collect::<Vec<_>>().map(|lines| {
        let mut instrs = Vec::new();
        let mut defs = HashMap::new();

        for line in lines {
            match line {
                Line::Instruction(real) => instrs.push(real),
                Line::Pseudo(pseudo) => instrs.extend(pseudo),
                Line::Label(def) => {
                    defs.insert(def, instrs.len());
                    ()
                }
            }
        }

        (instrs, defs)
    })
}
