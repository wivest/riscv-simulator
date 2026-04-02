use super::Processor;
use crate::instruction::Instruction;
use crate::names::*;

mod btype;
mod itype;
mod jtype;
mod rtype;
mod stype;
mod utype;

impl Instruction<i32, i32> {
    pub fn execute(&self, cpu: &mut Processor) -> bool {
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
            Instruction::System(_) => return false,
        }
        true
    }
}
