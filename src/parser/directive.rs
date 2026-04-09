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
        .ignore_then(number(8))
        .map(|n: u8| Directive::Byte(n))
}

fn byte2<'src>() -> impl Parser<'src, &'src str, Directive> {
    just(".byte")
        .ignore_then(number(16))
        .map(|n: u16| Directive::Byte2(n))
}

fn byte4<'src>() -> impl Parser<'src, &'src str, Directive> {
    just(".byte")
        .ignore_then(number(32))
        .map(|n: u32| Directive::Byte4(n))
}

fn byte8<'src>() -> impl Parser<'src, &'src str, Directive> {
    just(".byte")
        .ignore_then(number(64)) // FIXME: i32 is not capable of storing i64
        .map(|n: u64| Directive::Byte8(n))
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
