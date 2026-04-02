pub use crate::names::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction<I, O> {
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
        imm: I,
    },
    BType {
        name: BType,
        rs1: usize,
        rs2: usize,
        offset: O,
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
        imm: I,
    },
    JType {
        name: JType,
        rd: usize,
        imm: O,
    },
    System(System),
}
