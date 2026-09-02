use super::Processor;
use crate::language::instruction::Instruction;
use crate::language::names::*;

mod btype;
mod itype;
mod jtype;
mod rtype;
mod stype;
mod utype;

impl Instruction<i32, i32> {
    pub fn execute(&self, cpu: &mut Processor) {
        match *self {
            Instruction::BType {
                name,
                rs1,
                rs2,
                offset,
            } => name.execute(cpu, rs1, rs2, offset),
            Instruction::IType { name, rd, rs, imm } => name.execute(cpu, rd, rs, imm),
            Instruction::JType { name, rd, imm } => name.execute(cpu, rd, imm),
            Instruction::RType { name, rd, rs1, rs2 } => name.execute(cpu, rd, rs1, rs2),
            Instruction::SType {
                name,
                rs1,
                rs2,
                imm,
            } => name.execute(cpu, rs1, rs2, imm),
            Instruction::UType { name, rd, imm } => name.execute(cpu, rd, imm),
            Instruction::Ebreak => (),
        }
    }
}
