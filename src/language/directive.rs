use chumsky::span::SimpleSpan;

#[derive(Debug, PartialEq, Clone)]
pub enum Byte<'src> {
    Value(u8),
    Address(u32, &'src str, SimpleSpan),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Directive<'src> {
    Org(u32),
    Equ(String, u32),
    Unaligned(Vec<Byte<'src>>),
    Aligned(u32, Vec<Byte<'src>>),
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
