#[derive(Debug, PartialEq, Clone)]
pub enum Byte {
    Value(u8),
    Address(u32, String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Directive {
    Org(u32),
    Equ(String, u32),
    Unaligned(Vec<Byte>),
    Aligned(u32, Vec<Byte>),
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
