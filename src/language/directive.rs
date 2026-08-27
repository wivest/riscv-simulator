#[derive(Debug, PartialEq, Clone)]
pub enum Byte {
    Value(u8),
    Address(usize, String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Directive {
    Org(usize),
    Unaligned(Vec<Byte>),
    Aligned(usize, Vec<Byte>),
    Section(SectionName),
    Ignore,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SectionName {
    Text,
    Data,
    Rodata,
    Bss,
}
