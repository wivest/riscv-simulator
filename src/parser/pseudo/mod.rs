use super::common::*;
use chumsky::prelude::*;
pub use instruction::Pseudo;

mod instruction;

fn utype<'src>(
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Pseudo> {
    prefix
        .ignore_then(register().then_ignore(just(",")).then(immediate(32)))
        .map(move |(rd, imm)| Pseudo::Li { rd, imm })
}

pub fn pseudo_instructions<'src>() -> impl Parser<'src, &'src str, Pseudo> {
    let li = utype(just("li"));

    choice((li,))
}
