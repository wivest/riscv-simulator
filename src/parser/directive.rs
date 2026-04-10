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

fn byte<'src>() -> impl Parser<'src, &'src str, Directive> {
    just(".byte")
        .ignore_then(list(number(8), number(8)))
        .map(|v| Directive::Unaligned(v))
}

fn byte2<'src>() -> impl Parser<'src, &'src str, Directive> {
    just(".byte2")
        .ignore_then(list(number(16), number(16)))
        .map(|v: Vec<u16>| {
            Directive::Unaligned(v.into_iter().map(|n| n.to_ne_bytes()).flatten().collect())
        })
}

fn byte4<'src>() -> impl Parser<'src, &'src str, Directive> {
    just(".byte4")
        .ignore_then(list(number(32), number(32)))
        .map(|v: Vec<u32>| {
            Directive::Unaligned(v.into_iter().map(|n| n.to_ne_bytes()).flatten().collect())
        })
}

fn byte8<'src>() -> impl Parser<'src, &'src str, Directive> {
    just(".byte8")
        .ignore_then(list(number(64), number(64)))
        .map(|v: Vec<u64>| {
            Directive::Unaligned(v.into_iter().map(|n| n.to_ne_bytes()).flatten().collect())
        })
}

pub fn dirs<'src>() -> impl Parser<'src, &'src str, Directive> {
    choice((org(), asciz(), byte(), byte2(), byte4(), byte8()))
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
