use super::immediate::Immediate;
pub use crate::names::*;

#[derive(Debug, PartialEq)]
pub enum Instruction<'a> {
    RType {
        name: RType,
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    IType {
        name: IType,
        rd: usize,
        rs: usize,
        imm: i32,
    },
    BType {
        name: BType,
        rs1: usize,
        rs2: usize,
        offset: Immediate<'a>,
    },
    SType {
        name: SType,
        rs1: usize,
        rs2: usize,
        imm: i32,
    },
    UType {
        name: UType,
        rd: usize,
        imm: i32,
    },
    JType {
        name: JType,
        rd: usize,
        imm: Immediate<'a>,
    },
    System(System),
}
