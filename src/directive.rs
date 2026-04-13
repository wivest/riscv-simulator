#[derive(Debug, PartialEq)]
pub enum Directive {
    Org(usize),
    Asciz(String),
    Unaligned(Vec<u8>),
    Aligned(usize, Vec<u8>),
}
