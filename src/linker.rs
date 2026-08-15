use crate::language::{
    instruction::Instruction::{self, *},
    token::{Definition, Immediate, Offset, Reference},
    word::Word,
};

use crate::parser::Section;
use crate::processor::memory::Memory;
use std::collections::HashMap;

pub struct Linker<'a>(HashMap<usize, Word<Immediate<'a>, Offset<'a>>>);

impl<'a> Linker<'a> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn import_section(&mut self, sect: Section<Immediate<'a>, Offset<'a>>) {
        self.0.extend(sect.content);
    }

    pub fn link(self, defs: &'a HashMap<Definition<'a>, usize>) -> Memory<i32, i32> {
        Memory::from(
            self.0
                .into_iter()
                .map(|(div4, word)| {
                    let word = match word {
                        Word::Instruction(i) => {
                            Word::Instruction(translate_instr(i, div4 * 4, defs))
                        }
                        Word::Value(v) => Word::Value(v),
                    };
                    (div4, word)
                })
                .collect(),
        )
    }
}

pub fn translate_instr(
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
