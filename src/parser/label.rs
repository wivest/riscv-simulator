use chumsky::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Label {
    Reference(String),
    Definition(String),
}

pub fn label_ref<'src>() -> impl Parser<'src, &'src str, Label> {
    text::ascii::ident().map(|label: &str| Label::Reference(label.to_owned()))
}

pub fn label_def<'src>() -> impl Parser<'src, &'src str, Label> {
    text::ascii::ident()
        .then_ignore(just(":"))
        .map(|label: &str| Label::Definition(label.to_owned()))
}
