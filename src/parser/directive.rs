use crate::directive::Directive;
use crate::parser::common::*;

fn org<'src>() -> impl Parser<'src, &'src str, Directive> {
    just(".org")
        .ignore_then(number(32))
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

fn unaligned<'src, T: TryFrom<i64> + Default, const N: usize, F: Fn(T) -> [u8; N] + 'src>(
    dir: &'src str,
    bytes: u32,
    to_bytes: F,
) -> impl Parser<'src, &'src str, Directive> {
    just(dir)
        .ignore_then(list(number(bytes * 8), number(bytes * 8)))
        .map(move |v: Vec<T>| {
            Directive::Unaligned(v.into_iter().flat_map(|n| to_bytes(n).to_vec()).collect())
        })
}

pub fn dirs<'src>() -> impl Parser<'src, &'src str, Directive> {
    choice((
        org(),
        asciz(),
        unaligned(".byte", 1, u8::to_ne_bytes),
        unaligned(".2byte", 2, u16::to_ne_bytes),
        unaligned(".4byte", 4, u32::to_ne_bytes),
        unaligned(".8byte", 8, u64::to_ne_bytes),
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
}
