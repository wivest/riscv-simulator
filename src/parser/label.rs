use chumsky::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Reference(String);
#[derive(Debug, PartialEq)]
pub struct Definition(String);

pub fn label_ref<'src>() -> impl Parser<'src, &'src str, Reference> {
    text::ascii::ident().map(|label: &str| Reference(label.to_owned()))
}

pub fn label_def<'src>() -> impl Parser<'src, &'src str, Definition> {
    text::ascii::ident()
        .then_ignore(just(":"))
        .map(|label: &str| Definition(label.to_owned()))
}
