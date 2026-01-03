use crate::instructions::*;
use crate::parser::immediate::Immediate;

#[derive(Debug, PartialEq)]
pub enum Instruction {
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
        offset: i32,
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
        imm: i32,
    },
}

pub enum InstructionExtra {
    BType {
        name: BType,
        rs1: usize,
        rs2: usize,
        offset: Immediate,
    },
}
