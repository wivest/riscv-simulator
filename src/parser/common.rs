use chumsky::prelude::*;

pub fn digits<'src>(radix: u32) -> impl Parser<'src, &'src str, i32> {
    text::int(radix).map(move |s: &'src str| i32::from_str_radix(s, radix).unwrap())
}

fn number_radix<'src>(radix: u32, bits: u32) -> impl Parser<'src, &'src str, i32> {
    digits(radix)
        .filter(move |n| 0u32.leading_zeros() - n.leading_zeros() <= bits)
        .h_padded()
}

pub fn number<'src>(bits: u32) -> impl Parser<'src, &'src str, i32> {
    let sign = just("-").map(|_| -1).or(empty().map(|_| 1));
    let bin = just("0b").ignore_then(number_radix(2, bits));
    let oct = just("0o").ignore_then(number_radix(8, bits));
    let hex = just("0x").ignore_then(number_radix(16, bits));
    let dec = number_radix(10, bits);
    let imm = choice((bin, oct, hex, dec));

    sign.then_ignore(text::whitespace())
        .then(imm)
        .map(|(s, n)| s * n)
        .h_padded()
}

pub trait HPadded<'src, O>: Parser<'src, &'src str, O> + Sized {
    fn h_padded(self) -> impl Parser<'src, &'src str, O> {
        let h_whitespace = text::newline()
            .not()
            .ignore_then(text::whitespace().exactly(1))
            .repeated()
            .ignored();
        self.padded_by(h_whitespace.then(comment().or_not()))
    }
}

impl<'src, O, P> HPadded<'src, O> for P where P: Parser<'src, &'src str, O> {}

fn comment<'src>() -> impl Parser<'src, &'src str, ()> {
    let content = text::newline()
        .not()
        .ignore_then(any())
        .repeated()
        .ignored();
    choice((just("#"), just("//"))).ignore_then(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment() {
        let result = comment().parse("// this is slash comment");
        assert_eq!(result.unwrap(), ());
        let result = comment().parse("# this is hash comment");
        assert_eq!(result.unwrap(), ());
        let result = comment().parse("// this is\nnewline comment");
        assert_eq!(result.has_errors(), true);
    }

    #[test]
    fn test_h_padded() {
        let result = just("just").h_padded().parse(" \njust\n ");
        assert_eq!(result.has_errors(), true);
        let result = just("just").h_padded().parse("  just # comment");
        assert_eq!(result.has_output(), true);
        assert_eq!(result.unwrap(), "just");
    }
}
