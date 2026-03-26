use super::immediate::{Immediate, Offset};
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
        imm: Immediate<'a>,
    },
    BType {
        name: BType,
        rs1: usize,
        rs2: usize,
        offset: Offset<'a>,
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
        imm: Immediate<'a>,
    },
    JType {
        name: JType,
        rd: usize,
        imm: Offset<'a>,
    },
    System(System),
}
