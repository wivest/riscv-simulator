use crate::instruction::Instruction;
use crate::parser::token::{Definition, Immediate, Offset, Reference};
use std::collections::HashMap;

pub fn translate(
    instrs: Vec<(usize, Instruction<Immediate, Offset>)>,
    defs: HashMap<Definition, usize>,
) -> Vec<(usize, Instruction<i32, i32>)> {
    instrs
        .into_iter()
        .map(|(addr, instr)| {
            (
                addr,
                match instr {
                    Instruction::BType {
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
                        Instruction::BType {
                            name,
                            rs1,
                            rs2,
                            offset,
                        }
                    }
                    Instruction::IType { name, rd, rs, imm } => Instruction::IType {
                        name,
                        rd,
                        rs,
                        imm: calc_imm(imm, &defs),
                    },
                    Instruction::JType { name, rd, imm } => {
                        let imm = match imm {
                            Offset::Label(Reference(l)) => {
                                *defs.get(&Definition(l)).unwrap_or(&0) as i32 - addr as i32
                            }
                            Offset::Value(v) => v,
                        };
                        Instruction::JType { name, rd, imm }
                    }
                    Instruction::RType { name, rd, rs1, rs2 } => {
                        Instruction::RType { name, rd, rs1, rs2 }
                    }
                    Instruction::SType {
                        name,
                        rs1,
                        rs2,
                        imm,
                    } => Instruction::SType {
                        name,
                        rs1,
                        rs2,
                        imm: calc_imm(imm, &defs),
                    },
                    Instruction::UType { name, rd, imm } => Instruction::UType {
                        name,
                        rd,
                        imm: calc_imm(imm, &defs),
                    },
                    Instruction::System(sys) => Instruction::System(sys),
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
