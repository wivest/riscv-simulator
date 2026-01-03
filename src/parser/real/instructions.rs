use crate::instructions::*;
use crate::{parser::immediate::Immediate, processor::Processor};

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

impl Instruction {
    pub fn execute(&self, cpu: &mut Processor) {
        match *self {
            Instruction::RType { name, rd, rs1, rs2 } => name.execute(cpu, rd, rs1, rs2),
            Instruction::IType { name, rd, rs, imm } => name.execute(cpu, rd, rs, imm),
            Instruction::BType {
                name,
                rs1,
                rs2,
                offset,
            } => name.execute(cpu, rs1, rs2, offset),
            Instruction::SType {
                name,
                rs1,
                rs2,
                imm,
            } => name.execute(cpu, rs1, rs2, imm),
            Instruction::JType { name, rd, imm } => name.execute(cpu, rd, imm),
            Instruction::UType { name, rd, imm } => name.execute(cpu, rd, imm),
        }
    }
}
