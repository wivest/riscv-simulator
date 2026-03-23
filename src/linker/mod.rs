use crate::parser::immediate::Immediate;
use crate::parser::label::{Definition, Reference};
use crate::parser::real::instructions::Instruction as ParsInstr;
use crate::processor::instructions::Instruction as ProcInstr;
use std::collections::HashMap;

pub fn translate(instrs: Vec<ParsInstr>, defs: HashMap<Definition, usize>) -> Vec<ProcInstr> {
    instrs
        .iter()
        .enumerate()
        .map(|(i, instr)| match *instr {
            ParsInstr::BType {
                name,
                rs1,
                rs2,
                offset,
            } => {
                let offset = match offset {
                    Immediate::Label(Reference(l)) => {
                        (*defs.get(&Definition(l)).unwrap_or(&0) as i32 - i as i32) * 4
                    }
                    Immediate::Value(v) => v,
                };
                ProcInstr::BType {
                    name,
                    rs1,
                    rs2,
                    offset,
                }
            }
            ParsInstr::IType { name, rd, rs, imm } => ProcInstr::IType { name, rd, rs, imm },
            ParsInstr::JType { name, rd, imm } => {
                let imm = match imm {
                    Immediate::Label(Reference(l)) => {
                        (*defs.get(&Definition(l)).unwrap_or(&0) as i32 - i as i32) * 4
                    }
                    Immediate::Value(v) => v,
                };
                ProcInstr::JType { name, rd, imm }
            }
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
