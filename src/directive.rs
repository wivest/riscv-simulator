#[derive(Debug, PartialEq)]
pub enum Directive {
    Org(usize),
    Unaligned(Vec<u8>),
    Aligned(usize, Vec<u8>),
    Section(Section),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Section {
    Text,
    Data,
    Rodata,
    Bss,
}
