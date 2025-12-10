use chumsky::prelude::*;

fn number<'src, T: std::str::FromStr>(radix: u32) -> impl Parser<'src, &'src str, T> {
    text::int(radix).map(|s: &'src str| s.parse::<T>().ok().unwrap())
}

pub fn register<'src>() -> impl Parser<'src, &'src str, usize> {
    let index = just("x").ignore_then(number(10)).filter(|n| *n <= 31);

    let zero = just("zero").map(|_| 0);
    let ra = just("ra").map(|_| 1);
    let sp = just("sp").map(|_| 2);
    let gp = just("gp").map(|_| 3);
    let tp = just("tp").map(|_| 4);
    let fp = just("fp").map(|_| 8);

    let temporary = just("t")
        .ignore_then(number::<usize>(10))
        .filter(|n| *n <= 6)
        .map(|n| if n <= 2 { n + 5 } else { n + 25 });

    let saved = just("s")
        .ignore_then(number::<usize>(10))
        .filter(|n| *n <= 11)
        .map(|n| if n <= 1 { n + 8 } else { n + 16 });

    let argument = just("a")
        .ignore_then(number::<usize>(10))
        .filter(|n| *n <= 7)
        .map(|n| n + 10);

    choice((index, zero, ra, sp, gp, tp, fp, temporary, saved, argument)).padded()
}

pub fn immediate<'src>(bits: u32) -> impl Parser<'src, &'src str, i32> {
    let sign = just("-").map(|_| -1).or(empty().map(|_| 1));
    sign.then_ignore(text::whitespace())
        .then(number::<i32>(10))
        .filter(move |(_, n)| 0u32.leading_zeros() - n.leading_zeros() <= bits)
        .map(|(s, n)| s * n)
        .padded()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_x() {
        let result = register().parse("x10");
        assert_eq!(result.unwrap(), 10);
    }

    #[test]
    fn test_register_name() {
        let zero = register().parse("zero");
        let ra = register().parse("ra");
        let sp = register().parse("sp");
        let gp = register().parse("gp");
        let tp = register().parse("tp");
        let fp = register().parse("fp");

        assert_eq!(zero.unwrap(), 0);
        assert_eq!(ra.unwrap(), 1);
        assert_eq!(sp.unwrap(), 2);
        assert_eq!(gp.unwrap(), 3);
        assert_eq!(tp.unwrap(), 4);
        assert_eq!(fp.unwrap(), 8);
    }

    #[test]
    fn test_register_name_index() {
        let t6 = register().parse("t6");
        let a7 = register().parse("a7");
        let s11 = register().parse("s11");

        assert_eq!(t6.unwrap(), 31);
        assert_eq!(a7.unwrap(), 17);
        assert_eq!(s11.unwrap(), 27);

        let t7 = register().parse("t7");
        let a8 = register().parse("a8");
        let s12 = register().parse("s12");

        assert_eq!(t7.has_errors(), true);
        assert_eq!(a8.has_errors(), true);
        assert_eq!(s12.has_errors(), true);
    }

    #[test]
    fn test_immediate() {
        let result = immediate(32).parse("42");
        assert_eq!(result.unwrap(), 42);
        let result = immediate(32).parse("-42");
        assert_eq!(result.unwrap(), -42);
        let result = immediate(32).parse("- 42");
        assert_eq!(result.unwrap(), -42);

        let result = immediate(12).parse("4095");
        assert_eq!(result.unwrap(), 4095);
        let result = immediate(12).parse("4096");
        assert_eq!(result.has_errors(), true);
    }
}
