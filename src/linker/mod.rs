use crate::parser::immediate::Immediate;
use crate::parser::real::instructions::Instruction as ParsInstr;
use crate::processor::instructions::Instruction as ProcInstr;

pub fn translate(parsed: Vec<ParsInstr>) -> Vec<ProcInstr> {
    parsed
        .iter()
        .map(|i| match *i {
            ParsInstr::BType {
                name,
                rs1,
                rs2,
                offset: Immediate::Value(offset),
            } => ProcInstr::BType {
                name,
                rs1,
                rs2,
                offset,
            },
            ParsInstr::IType { name, rd, rs, imm } => ProcInstr::IType { name, rd, rs, imm },
            ParsInstr::JType {
                name,
                rd,
                imm: Immediate::Value(imm),
            } => ProcInstr::JType { name, rd, imm },
            ParsInstr::RType { name, rd, rs1, rs2 } => ProcInstr::RType { name, rd, rs1, rs2 },
            ParsInstr::SType {
                name,
                rs1,
                rs2,
                imm,
            } => ProcInstr::SType {
                name,
                rs1,
                rs2,
                imm,
            },
            ParsInstr::UType { name, rd, imm } => ProcInstr::UType { name, rd, imm },
            _ => ProcInstr::IType {
                name: crate::names::IType::Addi,
                rd: 0,
                rs: 0,
                imm: 0,
            }, // TODO: resolve labels, remove NOP
        })
        .collect()
}
