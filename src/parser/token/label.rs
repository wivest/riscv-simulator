use chumsky::error::RichReason;

use crate::language::token::{Definition, Reference};

use crate::parser::common::*;

pub fn label_ref<'src>() -> impl StrParser<'src, Reference<'src>> {
    text::ascii::ident()
        .inline()
        .map(|label: &str| Reference(label))
        .map_err(|e| {
            Rich::custom(
                *e.span(),
                format!("expected symbol, found {}", e.found().unwrap_or(&'_')),
            )
        })
}

pub fn label_def<'src>() -> impl StrParser<'src, Definition<'src>> {
    text::ascii::ident()
        .inline()
        .then_ignore(just(":").map_err(|e: Rich<'_, char>| {
            Rich::custom(*e.span(), "expected ':' after label definition")
        }))
        .inline()
        .map(|label: &str| Definition(label))
        .map_err(|e| match *e.reason() {
            RichReason::Custom(_) => e,
            _ => Rich::custom(*e.span(), "expected label definition"),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_reference() {
        let result = label_ref().parse("label");
        assert_eq!(result.unwrap(), Reference("label"));
        let result = label_ref().parse("  label // comment");
        assert_eq!(result.unwrap(), Reference("label"));
        let result = label_ref().parse("__4lphanuM");
        assert_eq!(result.unwrap(), Reference("__4lphanuM"));
        let result = label_ref().parse("42");
        assert_eq!(result.has_errors(), true);
        let result = label_ref().parse("label:");
        assert_eq!(result.has_errors(), true);
    }

    #[test]
    fn label_definition() {
        let result = label_def().parse("label:");
        assert_eq!(result.unwrap(), Definition("label"));
        let result = label_def().parse("  label : // comment");
        assert_eq!(result.unwrap(), Definition("label"));
        let result = label_def().parse("__4lphanuM:");
        assert_eq!(result.unwrap(), Definition("__4lphanuM"));
        let result = label_def().parse("42:");
        assert_eq!(result.has_errors(), true);
        let result = label_def().parse("label");
        assert_eq!(result.has_errors(), true);
    }
}
