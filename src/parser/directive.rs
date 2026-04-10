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

fn unaligned<'src, const B: usize>(dir: &'src str) -> impl Parser<'src, &'src str, Directive> {
    let list = number_le_bytes(B as u32 * 8)
        .separated_by(just(','))
        .collect();
    just(dir).ignore_then(list).map(move |v: Vec<[u8; B]>| {
        Directive::Unaligned(v.into_iter().flat_map(|n| n.to_vec()).collect())
    })
}

pub fn dirs<'src>() -> impl Parser<'src, &'src str, Directive> {
    choice((
        org(),
        asciz(),
        unaligned::<1>(".byte"),
        unaligned::<2>(".2byte"),
        unaligned::<4>(".4byte"),
        unaligned::<8>(".8byte"),
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
        let result = unaligned::<1>(".byte").parse(".byte 42");
        assert_eq!(result.unwrap(), Directive::Unaligned(vec![42]));
        let result = unaligned::<1>(".byte").parse(".byte 0x88, 255, -1");
        assert_eq!(
            result.unwrap(),
            Directive::Unaligned(vec![0x88, 255, -1i8 as u8])
        );
    }
}
