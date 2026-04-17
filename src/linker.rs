use crate::instruction::Instruction::{self, *};
use crate::parser::token::{Definition, Immediate, Offset, Reference};
use std::collections::HashMap;

pub fn link(
    instrs: Vec<(usize, Instruction<Immediate, Offset>)>,
    defs: HashMap<Definition, usize>,
) -> Vec<(usize, Instruction<i32, i32>)> {
    instrs
        .into_iter()
        .map(|(addr, instr)| (addr, translate_instr(instr, addr, &defs)))
        .collect()
}

fn translate_instr(
    instr: Instruction<Immediate, Offset>,
    addr: usize,
    defs: &HashMap<Definition, usize>,
) -> Instruction<i32, i32> {
    let calc_offset = |offset| match offset {
        Offset::Label(Reference(l)) => *defs.get(&Definition(l)).unwrap_or(&0) as i32 - addr as i32,
        Offset::Value(v) => v,
    };
    let calc_imm = |imm| match imm {
        Immediate::Value(v) => v,
        Immediate::Upper(Reference(l)) => {
            let v = *defs.get(&Definition(l)).unwrap_or(&0) as i32;
            v >> 12
        }
        Immediate::Lower(Reference(l)) => {
            let v = *defs.get(&Definition(l)).unwrap_or(&0) as i32;
            v << 20 >> 20
        }
    };

    match instr {
        BType {
            name,
            rs1,
            rs2,
            offset,
        } => BType {
            name,
            rs1,
            rs2,
            offset: calc_offset(offset),
        },
        IType { name, rd, rs, imm } => IType {
            name,
            rd,
            rs,
            imm: calc_imm(imm),
        },
        JType { name, rd, imm } => JType {
            name,
            rd,
            imm: calc_offset(imm),
        },
        RType { name, rd, rs1, rs2 } => RType { name, rd, rs1, rs2 },
        SType {
            name,
            rs1,
            rs2,
            imm,
        } => SType {
            name,
            rs1,
            rs2,
            imm: calc_imm(imm),
        },
        UType { name, rd, imm } => UType {
            name,
            rd,
            imm: calc_imm(imm),
        },
        System(sys) => System(sys),
    }
}
