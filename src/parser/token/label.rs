use crate::parser::common::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Reference<'a>(pub &'a str);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Definition<'a>(pub &'a str);

pub fn label_ref<'src>() -> impl Parser<'src, &'src str, Reference<'src>> {
    text::ascii::ident()
        .h_padded()
        .map(|label: &str| Reference(label))
}

pub fn label_def<'src>() -> impl Parser<'src, &'src str, Definition<'src>> {
    text::ascii::ident()
        .h_padded()
        .then_ignore(just(":"))
        .h_padded()
        .map(|label: &str| Definition(label))
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
