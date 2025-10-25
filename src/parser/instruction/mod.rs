pub use btype::BType;
pub use itype::IType;
pub use rtype::RType;
pub use stype::SType;

use crate::processor::Processor;

mod btype;
mod itype;
mod rtype;
mod stype;

#[derive(Debug)]
pub enum Instruction {
    RType {
        name: RType,
        rd: i32,
        rs1: i32,
        rs2: i32,
    },
    IType {
        name: IType,
        rd: i32,
        rs: i32,
        imm: i32,
    },
    BType {
        name: BType,
        rs1: i32,
        rs2: i32,
        offset: i32,
    },
    SType {
        name: SType,
        rs1: i32,
        rs2: i32,
        imm: i32,
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
        }
    }
}
