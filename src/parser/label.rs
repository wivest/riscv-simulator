use chumsky::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Label {
    Reference(String),
    Definition(usize),
}

pub fn label_ref<'src>() -> impl Parser<'src, &'src str, Label> {
    text::ascii::ident().map(|label: &str| Label::Reference(label.to_owned()))
}
