use super::common::*;

use crate::language::directive::{Byte, Directive, SectionName};

fn org<'src>() -> impl StrParser<'src, Directive> {
    just(".org")
        .name_then(number(32, usize::from_le_bytes))
        .map(|at: usize| Directive::Org(at))
}

fn ascii<'src>() -> impl StrParser<'src, Directive> {
    let string = just('"')
        .ignore_then(none_of('"').repeated().collect())
        .then_ignore(just('"'));
    just(".ascii")
        .ignore_then(text::inline_whitespace())
        .ignore_then(string)
        .map(|s: String| Directive::Unaligned(s.bytes().map(|b| Byte::Value(b)).collect()))
}

fn asciz<'src>() -> impl StrParser<'src, Directive> {
    let string = just('"')
        .ignore_then(none_of('"').repeated().collect())
        .then_ignore(just('"'));
    choice((just(".asciz"), just(".string")))
        .ignore_then(text::inline_whitespace())
        .ignore_then(string)
        .map(|s: String| {
            Directive::Unaligned(
                s.bytes()
                    .chain(std::iter::once(0))
                    .map(|b| Byte::Value(b))
                    .collect(),
            )
        })
}

fn symbol<'src>(b: usize) -> impl StrParser<'src, Vec<Byte>> {
    text::ascii::ident()
        .map(move |s: &'src str| (0..b).map(|i| Byte::Address(i, s.to_owned())).collect())
        .inline()
}

fn bytes<'src, const B: usize>() -> impl StrParser<'src, Vec<Byte>> {
    choice((
        symbol(B),
        number_le_bytes(B as u32 * 8).map(|n: [u8; B]| n.map(|b| Byte::Value(b)).to_vec()),
    ))
    .separated_by(just(','))
    .collect()
    .map(|v: Vec<Vec<Byte>>| v.into_iter().flatten().collect())
}

fn unaligned<'src, const B: usize>(dir: &'src str) -> impl StrParser<'src, Directive> {
    just(dir)
        .name_then(bytes::<B>())
        .map(|list: Vec<Byte>| Directive::Unaligned(list))
}

fn aligned<'src, const B: usize>(dir: &'src str) -> impl StrParser<'src, Directive> {
    just(dir)
        .name_then(bytes::<B>())
        .map(|list: Vec<Byte>| Directive::Aligned(B, list))
}

fn section<'src>(sec: SectionName, name: &'src str) -> impl StrParser<'src, Directive> {
    just(".section")
        .name_then(empty())
        .or_not()
        .ignore_then(just(name))
        .map(move |_| Directive::Section(sec))
}

// ignores everything after a directive until newline
fn ignore<'src>(name: &'src str) -> impl StrParser<'src, Directive> {
    just(name)
        .then(text::newline().not().then(any()).repeated())
        .to(Directive::Ignore)
}

pub fn dirs<'src>() -> impl StrParser<'src, Directive> {
    choice((
        org(),
        ascii(),
        asciz(),
        unaligned::<1>(".byte"),
        unaligned::<2>(".2byte"),
        unaligned::<4>(".4byte"),
        unaligned::<8>(".8byte"),
        aligned::<2>(".half"),
        aligned::<2>(".short"),
        aligned::<4>(".word"),
        aligned::<8>(".dword"),
        section(SectionName::Text, ".text"),
        section(SectionName::Data, ".data"),
        section(SectionName::Rodata, ".rodata"),
        section(SectionName::Bss, ".bss"),
        ignore(".equ"),
        ignore(".set"),
        ignore(".orig"),
        ignore(".globl"),
        ignore(".end"),
        ignore(".ent"),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(v: Vec<u8>) -> Vec<Byte> {
        v.into_iter().map(|b| Byte::Value(b)).collect()
    }

    #[test]
    fn test_ascii() {
        let result = ascii().parse(".ascii\"hello world!\"");
        let expected = b"hello world!".iter().map(|b| Byte::Value(*b)).collect();
        assert_eq!(result.unwrap(), Directive::Unaligned(expected));

        let result = asciz().parse(".asciz \"hello world!\"");
        let expected = b"hello world!\0".iter().map(|b| Byte::Value(*b)).collect();
        assert_eq!(result.unwrap(), Directive::Unaligned(expected));
    }

    #[test]
    fn test_bytes() {
        let result = unaligned::<1>(".byte").parse(".byte 42");
        assert_eq!(result.unwrap(), Directive::Unaligned(vec![Byte::Value(42)]));
        let result = unaligned::<1>(".byte").parse(".byte 0x88, 255, -1");
        let expected = to_vec(vec![0x88, 255, -1i8 as u8]);
        assert_eq!(result.unwrap(), Directive::Unaligned(expected));

        let result = unaligned::<4>(".4byte").parse(".4byte 0x42cafe, -1");
        let expected = to_vec(vec![0xfe, 0xca, 0x42, 0x00, 0xff, 0xff, 0xff, 0xff]);
        assert_eq!(result.unwrap(), Directive::Unaligned(expected));
        let result = aligned::<4>(".word").parse(".word 0x42cafe, -1");
        let expected = to_vec(vec![0xfe, 0xca, 0x42, 0x00, 0xff, 0xff, 0xff, 0xff]);
        assert_eq!(result.unwrap(), Directive::Aligned(4, expected));
    }

    #[test]
    fn test_bytes_address() {
        let result = aligned::<4>(".word").parse(".word 0x42cafe, name");
        let mut expected = to_vec(vec![0xfe, 0xca, 0x42, 0x00]);
        let sym: Vec<Byte> = (0..4)
            .map(|i| Byte::Address(i, "name".to_owned()))
            .collect();
        expected.extend(sym);
        assert_eq!(result.unwrap(), Directive::Aligned(4, expected));

        let result = unaligned::<2>(".2byte").parse(".2byte 0xcafe, name");
        let mut expected = to_vec(vec![0xfe, 0xca]);
        let sym: Vec<Byte> = (0..2)
            .map(|i| Byte::Address(i, "name".to_owned()))
            .collect();
        expected.extend(sym);
        assert_eq!(result.unwrap(), Directive::Unaligned(expected));

        let result = aligned::<4>(".word").parse(".wordname");
        assert_eq!(result.has_errors(), true);
    }

    #[test]
    fn test_sections() {
        let result = section(SectionName::Text, ".text").parse(".text");
        assert_eq!(result.unwrap(), Directive::Section(SectionName::Text));
        let result = section(SectionName::Bss, ".bss").parse(". bss");
        assert_eq!(result.has_errors(), true);
        let result = ignore(".equ").parse(".equ some 4-rgumnt$");
        assert_eq!(result.unwrap(), Directive::Ignore);
    }
}
