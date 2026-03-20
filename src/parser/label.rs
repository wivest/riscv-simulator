use chumsky::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Label {
    Reference(String),
    Definition(usize),
}

pub fn label_str<'src>() -> impl Parser<'src, &'src str, &'src str> {
    text::ascii::ident()
}

pub fn label_str_def<'src>() -> impl Parser<'src, &'src str, &'src str> {
    text::ascii::ident().then_ignore(just(":"))
}
