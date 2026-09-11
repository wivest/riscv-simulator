use chumsky::error::RichReason;

use crate::language::token::{Definition, Reference};

use crate::parser::common::*;

pub fn label_ref<'src>() -> impl StrParser<'src, Reference<'src>> {
    text::ascii::ident()
        .map_with(|label: &str, ext| Reference(label, ext.span()))
        .map_err_with_state(|_, span, _| Rich::custom(span, String::from("expected symbol")))
}

pub fn label_def<'src>() -> impl StrParser<'src, Definition<'src>> {
    text::ascii::ident()
        .space()
        .then_ignore(just(":").map_err(|e: Rich<'_, char>| {
            Rich::custom(*e.span(), "expected ':' after label definition")
        }))
        .map(|label: &str| Definition(label))
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
        assert_eq!(result.unwrap(), Reference("label", SimpleSpan::from(0..5)));
        let result = label_ref().parse("__4lphanuM");
        assert_eq!(
            result.unwrap(),
            Reference("__4lphanuM", SimpleSpan::from(0..10))
        );
        let result = label_ref().parse("42");
        assert_eq!(result.has_errors(), true);
        let result = label_ref().parse("label:");
        assert_eq!(result.has_errors(), true);
    }

    #[test]
    fn test_definition() {
        let result = label_def().parse("label:");
        assert_eq!(result.unwrap(), Definition("label"));
        let result = label_def().parse("label \t:");
        assert_eq!(result.unwrap(), Definition("label"));
        let result = label_def().parse("__4lphanuM:");
        assert_eq!(result.unwrap(), Definition("__4lphanuM"));
        let result = label_def().parse("42:");
        assert_eq!(result.has_errors(), true);
        let result = label_def().parse("label");
        assert_eq!(result.has_errors(), true);
    }
}
