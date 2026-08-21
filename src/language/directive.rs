#[derive(Debug, PartialEq, Clone)]
pub enum Directive {
    Org(usize),
    Unaligned(Vec<u8>),
    Aligned(usize, Vec<u8>),
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
