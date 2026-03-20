use chumsky::prelude::*;

pub trait FromStrRadix: Sized {
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, std::num::ParseIntError>;
}

impl FromStrRadix for usize {
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, std::num::ParseIntError> {
        usize::from_str_radix(s, radix)
    }
}

impl FromStrRadix for i32 {
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, std::num::ParseIntError> {
        i32::from_str_radix(s, radix)
    }
}

pub fn number<'src, T: FromStrRadix>(radix: u32) -> impl Parser<'src, &'src str, T> {
    text::int(radix).map(move |s: &'src str| T::from_str_radix(s, radix).unwrap())
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
