use super::common::*;
use crate::directive::Directive;
use chumsky::prelude::*;

pub fn org<'src>() -> impl Parser<'src, &'src str, Directive> {
    just(".org")
        .ignore_then(number(32))
        .map(|at| Directive::Org(at as usize))
}
