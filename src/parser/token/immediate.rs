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

#[cfg(test)]
mod tests {
    use crate::language::token::Reference;

    use super::*;

    #[test]
    fn test_immediate12() {
        let result = immediate12().parse("0xff");
        assert_eq!(result.unwrap(), Immediate::Value(0xff));
        let result = immediate12().parse("-1");
        assert_eq!(result.unwrap(), Immediate::Value(-1));
        let result = immediate12().parse("0xfff");
        assert_eq!(result.unwrap(), Immediate::Value(-1));
        let result = immediate12().parse("0x1000");
        assert_eq!(result.has_errors(), true);
        let result = immediate12().parse("%lo(name)");
        assert_eq!(result.unwrap(), Immediate::Lower(Reference("name")));
    }

    #[test]
    fn test_immediate20() {
        let result = immediate20().parse("0xff");
        assert_eq!(result.unwrap(), Immediate::Value(0xff));
        let result = immediate20().parse("-1");
        assert_eq!(result.unwrap(), Immediate::Value(-1));
        let result = immediate20().parse("0xfffff");
        assert_eq!(result.unwrap(), Immediate::Value(-1));
        let result = immediate20().parse("0x100000");
        assert_eq!(result.has_errors(), true);
        let result = immediate20().parse("%hi(name)");
        assert_eq!(result.unwrap(), Immediate::Upper(Reference("name")));
    }

    #[test]
    fn test_offset() {
        let result = offset(12).parse("0xff");
        assert_eq!(result.unwrap(), Offset::Value(0xff));
        let result = offset(12).parse("-1");
        assert_eq!(result.unwrap(), Offset::Value(-1));
        let result = offset(12).parse("0xfff");
        assert_eq!(result.unwrap(), Offset::Value(-1));
        let result = offset(12).parse("0x1000");
        assert_eq!(result.has_errors(), true);
        let result = offset(12).parse("name");
        assert_eq!(result.unwrap(), Offset::Label(Reference("name")));
    }
}
