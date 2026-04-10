use crate::directive::Directive;
use crate::parser::common::*;

fn org<'src>() -> impl Parser<'src, &'src str, Directive> {
    just(".org")
        .ignore_then(number(32, usize::from_le_bytes))
        .map(|at: usize| Directive::Org(at))
}

fn asciz<'src>() -> impl Parser<'src, &'src str, Directive> {
    let string = just('"')
        .ignore_then(none_of('"').repeated().collect())
        .then_ignore(just('"'));
    just(".asciz")
        .ignore_then(text::inline_whitespace())
        .ignore_then(string)
        .map(|s| Directive::Asciz(s))
}

fn unaligned<
    'src,
    T: TryFrom<i64> + Default,
    const N: usize,
    FT: Fn(T) -> [u8; N] + 'src,
    FF: Fn([u8; N]) -> T,
>(
    dir: &'src str,
    bytes: u32,
    to_bytes: FT,
    from_le_bytes: FF,
) -> impl Parser<'src, &'src str, Directive> {
    just(dir)
        .ignore_then(
            number(bytes * 8, from_le_bytes)
                .separated_by(just(','))
                .collect(),
        )
        .map(move |v: Vec<T>| {
            Directive::Unaligned(v.into_iter().flat_map(|n| to_bytes(n).to_vec()).collect())
        })
}

pub fn dirs<'src>() -> impl Parser<'src, &'src str, Directive> {
    choice((
        org(),
        asciz(),
        unaligned(".byte", 1, i8::to_ne_bytes, i8::from_le_bytes),
        unaligned(".2byte", 2, i16::to_ne_bytes, i16::from_le_bytes),
        unaligned(".4byte", 4, i32::to_ne_bytes, i32::from_le_bytes),
        unaligned(".8byte", 8, i64::to_ne_bytes, i64::from_le_bytes),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asciz_dir() {
        let result = asciz().parse(".asciz \"hello world!\"");
        assert_eq!(result.unwrap(), Directive::Asciz("hello world!".to_owned()));
    }

    #[test]
    fn byte() {
        let result = unaligned(".byte", 1, i8::to_ne_bytes, i8::from_le_bytes).parse(".byte 42");
        assert_eq!(result.unwrap(), Directive::Unaligned(vec![42]));
        let result =
            unaligned(".byte", 1, i8::to_ne_bytes, i8::from_le_bytes).parse(".byte 0x88, 255, -1");
        assert_eq!(
            result.unwrap(),
            Directive::Unaligned(vec![0x88, 255, -1i8 as u8])
        );
    }
}
