#[derive(Debug, PartialEq)]
pub enum Directive {
    Org(usize),
    Unaligned(Vec<u8>),
    Aligned(usize, Vec<u8>),
    Section(SectionName),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SectionName {
    Text,
    Data,
    Rodata,
    Bss,
}
