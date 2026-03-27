use super::common::*;
use super::label::*;
use chumsky::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Offset<'a> {
    Value(i32),
    Label(Reference<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Immediate<'a> {
    Value(i32),
    Upper(Reference<'a>),
    Lower(Reference<'a>),
}

pub fn immediate12<'src>() -> impl Parser<'src, &'src str, Immediate<'src>> {
    let imm = number(12).map(|imm| Immediate::Value(imm));
    let lower = just("%lo(")
        .ignore_then(label_ref())
        .then_ignore(just(")"))
        .map(|label| Immediate::Lower(label))
        .h_padded();

    choice((imm, lower))
}

pub fn immediate20<'src>() -> impl Parser<'src, &'src str, Immediate<'src>> {
    let imm = number(20).map(|imm| Immediate::Value(imm));
    let lower = just("%hi(")
        .ignore_then(label_ref())
        .then_ignore(just(")"))
        .map(|label| Immediate::Upper(label))
        .h_padded();

    choice((imm, lower))
}

pub fn offset<'src>(bits: u32) -> impl Parser<'src, &'src str, Offset<'src>> {
    let imm = number(bits).map(|imm| Offset::Value(imm));
    let label = label_ref().map(|label| Offset::Label(label));
    choice((imm, label))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immediate() {
        let result = number(32).parse("42");
        assert_eq!(result.unwrap(), 42);
        let result = number(32).parse("-42");
        assert_eq!(result.unwrap(), -42);
        let result = number(32).parse("- 42");
        assert_eq!(result.unwrap(), -42);

        let result = number(12).parse("4095");
        assert_eq!(result.unwrap(), 4095);
        let result = number(12).parse("4096");
        assert_eq!(result.has_errors(), true);
    }

    #[test]
    fn test_immediate_base() {
        let result = number(32).parse("0b10");
        assert_eq!(result.unwrap(), 0b10);
        let result = number(32).parse("0o42");
        assert_eq!(result.unwrap(), 0o42);
        let result = number(32).parse("0x42");
        assert_eq!(result.unwrap(), 0x42);
    }
}
