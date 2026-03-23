use crate::parser::common::HPadded;
use chumsky::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Reference(String);
#[derive(Debug, PartialEq)]
pub struct Definition(String);

pub fn label_ref<'src>() -> impl Parser<'src, &'src str, Reference> {
    text::ascii::ident()
        .h_padded()
        .map(|label: &str| Reference(label.to_owned()))
}

pub fn label_def<'src>() -> impl Parser<'src, &'src str, Definition> {
    text::ascii::ident()
        .h_padded()
        .then_ignore(just(":"))
        .h_padded()
        .map(|label: &str| Definition(label.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_reference() {
        let result = label_ref().parse("label");
        assert_eq!(result.unwrap(), Reference("label".to_owned()));
        let result = label_ref().parse("  label // comment");
        assert_eq!(result.unwrap(), Reference("label".to_owned()));
        let result = label_ref().parse("__4lphanuM");
        assert_eq!(result.unwrap(), Reference("__4lphanuM".to_owned()));
        let result = label_ref().parse("42");
        assert_eq!(result.has_errors(), true);
        let result = label_ref().parse("label:");
        assert_eq!(result.has_errors(), true);
    }

    #[test]
    fn label_definition() {
        let result = label_def().parse("label:");
        assert_eq!(result.unwrap(), Definition("label".to_owned()));
        let result = label_def().parse("  label : // comment");
        assert_eq!(result.unwrap(), Definition("label".to_owned()));
        let result = label_def().parse("__4lphanuM:");
        assert_eq!(result.unwrap(), Definition("__4lphanuM".to_owned()));
        let result = label_def().parse("42:");
        assert_eq!(result.has_errors(), true);
        let result = label_def().parse("label");
        assert_eq!(result.has_errors(), true);
    }
}
