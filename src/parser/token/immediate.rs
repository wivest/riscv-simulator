use super::label_ref;

use crate::language::token::{Immediate, Offset};
use crate::parser::common::*;

pub fn immediate12<'src>() -> impl StrParser<'src, Immediate<'src>> {
    let imm = number(12, i32::from_le_bytes).map(|imm| Immediate::Value(imm));
    let lower = just("%lo(")
        .ignore_then(label_ref())
        .then_ignore(just(")"))
        .map(|label| Immediate::Lower(label));

    choice((imm, lower)).inline()
}

pub fn immediate20<'src>() -> impl StrParser<'src, Immediate<'src>> {
    let imm = number(20, i32::from_le_bytes).map(|imm| Immediate::Value(imm));
    let lower = just("%hi(")
        .ignore_then(label_ref())
        .then_ignore(just(")"))
        .map(|label| Immediate::Upper(label));

    choice((imm, lower)).inline()
}

pub fn offset<'src>(bits: u32) -> impl StrParser<'src, Offset<'src>> {
    let imm = number(bits, i32::from_le_bytes).map(|imm| Offset::Value(imm));
    let label = label_ref().map(|label| Offset::Label(label));
    choice((imm, label)).inline()
}
