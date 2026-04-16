#[derive(Debug, PartialEq)]
pub enum Directive {
    Org(usize),
    Unaligned(Vec<u8>),
    Aligned(usize, Vec<u8>),
}
