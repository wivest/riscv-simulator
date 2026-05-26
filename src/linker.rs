use crate::instruction::Instruction::{self, *};
use crate::parser::token::{Definition, Immediate, Offset, Reference};
use crate::processor::memory::{Memory, Sect, Word};
use std::collections::HashMap;

pub fn link_section<'a>(
    section: Sect<Immediate, Offset>,
    defs: &'a HashMap<Definition, usize>,
) -> Memory<i32, i32> {
    let mut linked = Memory::new();
    for (div4, word) in section.memory.words {
        let addr = div4 * 4;
        match word {
            Word::Instruction(i) => {
                let instr = translate_instr(i, addr, &defs);
                linked.store_instr(addr, instr);
            }
            Word::Value(v) => {
                linked.set_word(addr, v);
            }
        }
    }
    linked
}

fn translate_instr(
    instr: Instruction<Immediate, Offset>,
    addr: usize,
    defs: &HashMap<Definition, usize>,
) -> Instruction<i32, i32> {
    let resolve = |l| *defs.get(&Definition(l)).unwrap_or(&0) as i32;
    let calc_offset = |offset| match offset {
        Offset::Label(Reference(l)) => resolve(l) - addr as i32,
        Offset::Value(v) => v,
    };
    let calc_imm = |imm| match imm {
        Immediate::Value(v) => v,
        Immediate::Upper(Reference(l)) => resolve(l) >> 12,
        Immediate::Lower(Reference(l)) => resolve(l) << 20 >> 20,
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
