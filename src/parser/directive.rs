use super::common::*;

use crate::language::directive::{Directive, SectionName};

fn org<'src>() -> impl StrParser<'src, Directive> {
    just(".org")
        .ignore_then(number(32, usize::from_le_bytes))
        .map(|at: usize| Directive::Org(at))
}

fn ascii<'src>() -> impl StrParser<'src, Directive> {
    let string = just('"')
        .ignore_then(none_of('"').repeated().collect())
        .then_ignore(just('"'));
    just(".ascii")
        .ignore_then(text::inline_whitespace())
        .ignore_then(string)
        .map(|s: String| Directive::Unaligned(s.bytes().collect()))
}

fn asciz<'src>() -> impl StrParser<'src, Directive> {
    let string = just('"')
        .ignore_then(none_of('"').repeated().collect())
        .then_ignore(just('"'));
    choice((just(".asciz"), just(".string")))
        .ignore_then(text::inline_whitespace())
        .ignore_then(string)
        .map(|s: String| Directive::Unaligned(s.bytes().chain(std::iter::once(0)).collect()))
}

fn unaligned<'src, const B: usize>(dir: &'src str) -> impl StrParser<'src, Directive> {
    let list = number_le_bytes(B as u32 * 8)
        .separated_by(just(','))
        .collect();
    just(dir).ignore_then(list).map(move |v: Vec<[u8; B]>| {
        Directive::Unaligned(v.into_iter().flat_map(|n| n.to_vec()).collect())
    })
}

fn aligned<'src, const B: usize>(dir: &'src str) -> impl StrParser<'src, Directive> {
    let list = number_le_bytes(B as u32 * 8)
        .separated_by(just(','))
        .collect();
    just(dir).ignore_then(list).map(move |v: Vec<[u8; B]>| {
        Directive::Aligned(B, v.into_iter().flat_map(|n| n.to_vec()).collect())
    })
}

fn section<'src>(sec: SectionName, name: &'src str) -> impl StrParser<'src, Directive> {
    just(".section")
        .ignore_then(text::inline_whitespace())
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

    #[test]
    fn test_ascii() {
        let result = ascii().parse(".ascii\"hello world!\"");
        let expected = b"hello world!".to_vec();
        assert_eq!(result.unwrap(), Directive::Unaligned(expected));

        let result = asciz().parse(".asciz \"hello world!\"");
        let expected = b"hello world!\0".to_vec();
        assert_eq!(result.unwrap(), Directive::Unaligned(expected));
    }

    #[test]
    fn test_bytes() {
        let result = unaligned::<1>(".byte").parse(".byte 42");
        assert_eq!(result.unwrap(), Directive::Unaligned(vec![42]));
        let result = unaligned::<1>(".byte").parse(".byte 0x88, 255, -1");
        assert_eq!(
            result.unwrap(),
            Directive::Unaligned(vec![0x88, 255, -1i8 as u8])
        );

        let result = unaligned::<4>(".4byte").parse(".4byte 0x42cafe, -1");
        assert_eq!(
            result.unwrap(),
            Directive::Unaligned(vec![0xfe, 0xca, 0x42, 0x00, 0xff, 0xff, 0xff, 0xff])
        );
        let result = aligned::<4>(".word").parse(".word 0x42cafe, -1");
        assert_eq!(
            result.unwrap(),
            Directive::Aligned(4, vec![0xfe, 0xca, 0x42, 0x00, 0xff, 0xff, 0xff, 0xff])
        );
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
