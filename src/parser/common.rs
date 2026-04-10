pub use chumsky::prelude::*;

pub fn digits<'src>(radix: u32) -> impl Parser<'src, &'src str, u64> {
    text::int(radix).map(move |s: &'src str| u64::from_str_radix(s, radix).unwrap())
}

fn char<'src>() -> impl Parser<'src, &'src str, i64> {
    just('\'')
        .ignore_then(none_of('\''))
        .then_ignore(just('\''))
        .filter(|c| *c <= u8::MAX as char)
        .map(|c| c as i64)
}

fn number_radix<'src>(radix: u32, bits: u32) -> impl Parser<'src, &'src str, i64> {
    digits(radix)
        .filter(move |n| 0u64.leading_zeros() - n.leading_zeros() <= bits)
        .map(|n| n as i64)
        .inline()
}

pub fn number_le_bytes<'src, const N: usize>(bits: u32) -> impl Parser<'src, &'src str, [u8; N]> {
    let neg = just('-')
        .ignore_then(text::inline_whitespace())
        .ignore_then(number_radix(10, bits - 1))
        .map(|n| -n);
    let pos = number_radix(10, bits);
    let dec = choice((neg, pos));

    let bin = just("0b").ignore_then(number_radix(2, bits));
    let oct = just("0o").ignore_then(number_radix(8, bits));
    let hex = just("0x").ignore_then(number_radix(16, bits));

    choice((bin, oct, hex, dec, char())) // dec must come AFTER prefixed
        .map(move |n| n.to_le_bytes()[..N].try_into().unwrap())
        .inline()
}

pub fn number<'src, O, const N: usize, F: Fn([u8; N]) -> O>(
    bits: u32,
    from_le_bytes: F,
) -> impl Parser<'src, &'src str, O> {
    number_le_bytes(bits).map(move |bytes| from_le_bytes(bytes))
}

pub trait Extended<'src, O>: Parser<'src, &'src str, O> + Sized {
    fn inline(self) -> impl Parser<'src, &'src str, O> {
        self.padded_by(text::inline_whitespace().then(comment().or_not()))
    }

    fn then_arg<OA, A: Parser<'src, &'src str, OA>>(
        self,
        arg: A,
    ) -> impl Parser<'src, &'src str, (O, OA)> {
        self.then_ignore(just(',')).then(arg)
    }

    fn index<OA, A: Parser<'src, &'src str, OA>>(
        self,
        arg: A,
    ) -> impl Parser<'src, &'src str, (O, OA)> {
        self.then_ignore(just('(')).then(arg).then_ignore(just(')'))
    }
}

impl<'src, O, P> Extended<'src, O> for P where P: Parser<'src, &'src str, O> {}

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
    fn test_digits() {
        // test for max possible value
        let result = digits(16).parse("FFFFFFFFFFFFFFFF");
        assert_eq!(result.unwrap(), u64::MAX);
    }

    #[test]
    fn test_number() {
        let result = number(32, i32::from_le_bytes).parse("42");
        assert_eq!(result.unwrap(), 42);
        let result = number(32, i32::from_le_bytes).parse("-42");
        assert_eq!(result.unwrap(), -42);
        let result = number(32, i32::from_le_bytes).parse("- 42");
        assert_eq!(result.unwrap(), -42);
        let result = number(32, i32::from_le_bytes).parse("-\n42");
        assert_eq!(result.has_errors(), true);

        let result = number(12, i16::from_le_bytes).parse("0xfff");
        assert_eq!(result.unwrap(), 0xfff);
        let result = number(12, i16::from_le_bytes).parse("0x1000");
        assert_eq!(result.has_errors(), true);
        let result = number(12, i16::from_le_bytes).parse("-2047");
        assert_eq!(result.unwrap(), -2047);
        let result = number(12, i16::from_le_bytes).parse("-2048");
        assert_eq!(result.has_errors(), true);
    }

    #[test]
    fn test_number_radix() {
        let result = number(32, i32::from_le_bytes).parse("0b10");
        assert_eq!(result.unwrap(), 0b10);
        let result = number(32, i32::from_le_bytes).parse("0o42");
        assert_eq!(result.unwrap(), 0o42);
        let result = number(32, i32::from_le_bytes).parse("0x42");
        assert_eq!(result.unwrap(), 0x42);
        let result = number(32, i32::from_le_bytes).parse("-0x42");
        assert_eq!(result.has_errors(), true);
    }

    #[test]
    fn test_char() {
        let result = number(8, u8::from_le_bytes).parse("'a'");
        assert_eq!(result.unwrap(), 'a' as u8);
        let result = char().parse("'a'");
        assert_eq!(result.unwrap(), 'a' as i64);
        let result = char().parse("'🚀'");
        assert_eq!(result.has_errors(), true);
    }

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
    fn test_inline() {
        let result = just("just").inline().parse(" \njust\n ");
        assert_eq!(result.has_errors(), true);
        let result = just("just").inline().parse("  just # comment");
        assert_eq!(result.has_output(), true);
        assert_eq!(result.unwrap(), "just");
    }
}
