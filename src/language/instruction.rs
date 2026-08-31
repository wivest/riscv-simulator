pub use super::names::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction<I, O> {
    RType {
        name: RType,
        rd: u32,
        rs1: u32,
        rs2: u32,
    },
    IType {
        name: IType,
        rd: u32,
        rs: u32,
        imm: I,
    },
    BType {
        name: BType,
        rs1: u32,
        rs2: u32,
        offset: O,
    },
    SType {
        name: SType,
        rs1: u32,
        rs2: u32,
        imm: I,
    },
    UType {
        name: UType,
        rd: u32,
        imm: I,
    },
    JType {
        name: JType,
        rd: u32,
        imm: O,
    },
    System(System),
}
