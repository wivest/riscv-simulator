#[derive(Debug, PartialEq)]
pub enum Directive {
    Org(usize),
    Asciz(String),
    Byte(u8),
    Byte2(u16),
    Byte4(u32),
    Byte8(u64),
}
