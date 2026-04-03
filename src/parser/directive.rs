use crate::directive::Directive;
use crate::parser::common::*;

fn org<'src>() -> impl Parser<'src, &'src str, Directive> {
    just(".org")
        .ignore_then(number(32))
        .map(|at| Directive::Org(at as usize))
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
        .map(|n| Directive::Byte(n as u8))
}

fn byte2<'src>() -> impl Parser<'src, &'src str, Directive> {
    just(".byte")
        .ignore_then(number(16))
        .map(|n| Directive::Byte2(n as u16))
}

fn byte4<'src>() -> impl Parser<'src, &'src str, Directive> {
    just(".byte")
        .ignore_then(number(32))
        .map(|n| Directive::Byte4(n as u32))
}

fn byte8<'src>() -> impl Parser<'src, &'src str, Directive> {
    just(".byte")
        .ignore_then(number(64)) // FIXME: i32 is not capable of storing i64
        .map(|n| Directive::Byte8(n as u64))
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
