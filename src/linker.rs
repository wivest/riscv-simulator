use crate::parser::grammar::Instruction as ParsInstr;
use crate::parser::immediate::{Immediate, Offset};
use crate::parser::label::{Definition, Reference};
use crate::processor::instructions::Instruction as ProcInstr;
use std::collections::HashMap;

pub fn translate(
    instrs: Vec<(usize, ParsInstr)>,
    defs: HashMap<Definition, usize>,
) -> Vec<(usize, ProcInstr)> {
    instrs
        .into_iter()
        .map(|(addr, instr)| {
            (
                addr,
                match instr {
                    ParsInstr::BType {
                        name,
                        rs1,
                        rs2,
                        offset,
                    } => {
                        let offset = match offset {
                            Offset::Label(Reference(l)) => {
                                *defs.get(&Definition(l)).unwrap_or(&0) as i32 - addr as i32
                            }
                            Offset::Value(v) => v,
                        };
                        ProcInstr::BType {
                            name,
                            rs1,
                            rs2,
                            offset,
                        }
                    }
                    ParsInstr::IType { name, rd, rs, imm } => ProcInstr::IType {
                        name,
                        rd,
                        rs,
                        imm: calc_imm(imm, &defs),
                    },
                    ParsInstr::JType { name, rd, imm } => {
                        let imm = match imm {
                            Offset::Label(Reference(l)) => {
                                *defs.get(&Definition(l)).unwrap_or(&0) as i32 - addr as i32
                            }
                            Offset::Value(v) => v,
                        };
                        ProcInstr::JType { name, rd, imm }
                    }
                    ParsInstr::RType { name, rd, rs1, rs2 } => {
                        ProcInstr::RType { name, rd, rs1, rs2 }
                    }
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
                    ParsInstr::UType { name, rd, imm } => ProcInstr::UType {
                        name,
                        rd,
                        imm: calc_imm(imm, &defs),
                    },
                    ParsInstr::System(sys) => ProcInstr::System(sys),
                },
            )
        })
        .collect()
}

fn calc_imm(imm: Immediate, defs: &HashMap<Definition, usize>) -> i32 {
    match imm {
        Immediate::Value(v) => v,
        Immediate::Upper(Reference(l)) => {
            let v = *defs.get(&Definition(l)).unwrap_or(&0) as i32;
            v >> 12
        }
        Immediate::Lower(Reference(l)) => {
            let v = *defs.get(&Definition(l)).unwrap_or(&0) as i32;
            v << 20 >> 20
        }
    }
}
