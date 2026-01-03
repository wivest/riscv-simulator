use crate::parser::real::instructions::Instruction as ParsInstr;
use crate::processor::instructions::Instruction as ProcInstr;

pub fn translate(parsed: Vec<ParsInstr>) -> Vec<ProcInstr> {
    parsed
        .iter()
        .map(|i| match i {
            ParsInstr::BType {
                name,
                rs1,
                rs2,
                offset,
            } => todo!(),
            ParsInstr::IType { name, rd, rs, imm } => todo!(),
            ParsInstr::JType { name, rd, imm } => todo!(),
            ParsInstr::RType { name, rd, rs1, rs2 } => todo!(),
            ParsInstr::SType {
                name,
                rs1,
                rs2,
                imm,
            } => todo!(),
            ParsInstr::UType { name, rd, imm } => todo!(),
        })
        .collect()
}
