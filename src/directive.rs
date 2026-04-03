#[derive(Debug, PartialEq)]
pub enum Directive {
    Org(usize),
    Asciz(String),
    Byte(u8),
}
