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
                offset,
            } => ProcInstr::BType {
                name,
                rs1,
                rs2,
                offset,
            },
            ParsInstr::IType { name, rd, rs, imm } => ProcInstr::IType { name, rd, rs, imm },
            ParsInstr::JType { name, rd, imm } => ProcInstr::JType { name, rd, imm },
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
        })
        .collect()
}
