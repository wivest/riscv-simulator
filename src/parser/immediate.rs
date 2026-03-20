use super::common::*;
use super::label::*;
use chumsky::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Immediate {
    Value(i32),
    Label(Label),
}

fn radix_immediate<'src>(radix: u32, bits: u32) -> impl Parser<'src, &'src str, i32> {
    number::<i32>(radix)
        .filter(move |n| 0u32.leading_zeros() - n.leading_zeros() <= bits)
        .h_padded()
}

pub fn immediate<'src>(bits: u32) -> impl Parser<'src, &'src str, i32> {
    let sign = just("-").map(|_| -1).or(empty().map(|_| 1));
    let bin = just("0b").ignore_then(radix_immediate(2, bits));
    let oct = just("0o").ignore_then(radix_immediate(8, bits));
    let hex = just("0x").ignore_then(radix_immediate(16, bits));
    let dec = radix_immediate(10, bits);
    let imm = choice((bin, oct, hex, dec));

    sign.then_ignore(text::whitespace())
        .then(imm)
        .map(|(s, n)| s * n)
        .h_padded()
}

pub fn offset<'src>(bits: u32) -> impl Parser<'src, &'src str, Immediate> {
    let imm = immediate(bits).map(|imm| Immediate::Value(imm));
    let label = label_ref().map(|label| Immediate::Label(label));
    choice((imm, label))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immediate() {
        let result = immediate(32).parse("42");
        assert_eq!(result.unwrap(), 42);
        let result = immediate(32).parse("-42");
        assert_eq!(result.unwrap(), -42);
        let result = immediate(32).parse("- 42");
        assert_eq!(result.unwrap(), -42);

        let result = immediate(12).parse("4095");
        assert_eq!(result.unwrap(), 4095);
        let result = immediate(12).parse("4096");
        assert_eq!(result.has_errors(), true);
    }

    #[test]
    fn test_immediate_base() {
        let result = immediate(32).parse("0b10");
        assert_eq!(result.unwrap(), 0b10);
        let result = immediate(32).parse("0o42");
        assert_eq!(result.unwrap(), 0o42);
        let result = immediate(32).parse("0x42");
        assert_eq!(result.unwrap(), 0x42);
    }
}
