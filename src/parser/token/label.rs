use chumsky::error::RichReason;

use crate::language::token::{Definition, Label, Reference};

use crate::parser::common::*;

pub fn label_ref<'src>() -> impl StrParser<'src, Reference<'src>> {
    text::ascii::ident()
        .map_with(|label: &str, ext| Reference(Label(label), ext.span()))
        .map_err_with_state(|_, span, _| Rich::custom(span, String::from("expected symbol")))
}

pub fn label_def<'src>() -> impl StrParser<'src, Definition<'src>> {
    text::ascii::ident()
        .space()
        .then_ignore(just(":").map_err(|e: Rich<'_, char>| {
            Rich::custom(*e.span(), "expected ':' after label definition")
        }))
        .map_with(|label, ext| Definition(Label(label), ext.span()))
        .map_err(|e| match *e.reason() {
            RichReason::Custom(_) => e,
            _ => Rich::custom(*e.span(), "invalid label definition"),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference() {
        let result = label_ref().parse("label");
        assert_eq!(
            result.unwrap(),
            Reference(Label("label"), SimpleSpan::from(0..5))
        );
        let result = label_ref().parse("__4lphanuM");
        assert_eq!(
            result.unwrap(),
            Reference(Label("__4lphanuM"), SimpleSpan::from(0..10))
        );
        let result = label_ref().parse("42");
        assert_eq!(result.has_errors(), true);
        let result = label_ref().parse("label:");
        assert_eq!(result.has_errors(), true);
    }

    #[test]
    fn test_definition() {
        let result = label_def().parse("label:");
        assert_eq!(
            result.unwrap(),
            Definition(Label("label"), SimpleSpan::from(0..6))
        );
        let result = label_def().parse("label \t:");
        assert_eq!(
            result.unwrap(),
            Definition(Label("label"), SimpleSpan::from(0..8))
        );
        let result = label_def().parse("__42a:");
        assert_eq!(
            result.unwrap(),
            Definition(Label("__42a"), SimpleSpan::from(0..6))
        );
        let result = label_def().parse("42:");
        assert_eq!(result.has_errors(), true);
        let result = label_def().parse("label");
        assert_eq!(result.has_errors(), true);
    }
}
