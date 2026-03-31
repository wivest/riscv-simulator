use crate::directive::Directive;
use crate::parser::common::*;

pub fn org<'src>() -> impl Parser<'src, &'src str, Directive> {
    just(".org")
        .ignore_then(number(32))
        .map(|at| Directive::Org(at as usize))
}

pub fn asciz<'src>() -> impl Parser<'src, &'src str, Directive> {
    let string = just('"')
        .ignore_then(none_of('"').repeated().collect())
        .then_ignore(just('"'));
    just(".asciz")
        .ignore_then(text::inline_whitespace())
        .ignore_then(string)
        .map(|s| Directive::Asciz(s))
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
