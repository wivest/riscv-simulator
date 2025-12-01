use super::{immediate, register};
use chumsky::prelude::*;
pub use instruction::Pseudo;

mod instruction;

fn utype<'src>(
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Pseudo> {
    prefix
        .ignore_then(register().then_ignore(just(",")).then(immediate(20)))
        .map(move |(rd, imm)| Pseudo::Li { rd, imm })
}
