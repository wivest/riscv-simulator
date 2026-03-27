use super::immediate::immediate;
use crate::directive::Directive;
use chumsky::prelude::*;

pub fn org<'src>() -> impl Parser<'src, &'src str, Directive> {
    just(".org")
        .ignore_then(immediate(32))
        .map(|at| Directive::Org(at as usize))
}
