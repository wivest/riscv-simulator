use crate::language::{
    instruction::Instruction::{self, *},
    token::{Definition, Immediate, Offset, Reference},
    word::Word,
};

use crate::parser::section::Section;
use crate::processor::memory::Memory;
use std::collections::HashMap;

pub struct Linker<'src> {
    defs: HashMap<Definition<'src>, usize>,
    memory: HashMap<usize, Word<Immediate<'src>, Offset<'src>>>,
    links: Vec<(usize, usize, String)>,
    equs: HashMap<String, u32>,
}

impl<'src> Linker<'src> {
    pub fn new() -> Self {
        Self {
            defs: HashMap::new(),
            memory: HashMap::new(),
            links: Vec::new(),
            equs: HashMap::new(),
        }
    }

    pub fn import_section(&mut self, sect: Section<'src, Immediate<'src>, Offset<'src>>) {
        for (at, word) in sect.content {
            self.memory.insert(sect.base / 4 + at, word);
        }
        for (def, at) in sect.defs {
            self.defs.insert(def, sect.base / 4 + at);
        }
        self.links.extend(sect.links);
        self.equs.extend(sect.equs);
    }

    pub fn link(self) -> Memory<i32, i32> {
        let mut result = Memory::from(
            self.memory
                .into_iter()
                .map(|(div4, word)| {
                    let word = match word {
                        Word::Instruction(i) => {
                            Word::Instruction(translate_instr(i, div4 * 4, &self.defs, &self.equs))
                        }
                        Word::Value(v) => Word::Value(v),
                    };
                    (div4, word)
                })
                .collect(),
        );
        for (at, b, link) in self.links {
            let addr = *self.defs.get(&Definition(&link)).unwrap();
            result.set(at, addr.to_le_bytes()[b]);
        }
        result
    }
}

pub fn translate_instr(
    instr: Instruction<Immediate, Offset>,
    addr: usize,
    defs: &HashMap<Definition, usize>,
    equs: &HashMap<String, u32>,
) -> Instruction<i32, i32> {
    let resolve = |l| *defs.get(&Definition(l)).unwrap_or(&0) as i32; // TODO: should be relative to pc
    let calc_offset = |offset| match offset {
        Offset::Label(Reference(l)) => resolve(l) - addr as i32,
        Offset::Value(v) => v,
    };
    let calc_imm = |imm| match imm {
        Immediate::Value(v) => v,
        Immediate::Upper(Reference(l)) => resolve(l) >> 12,
        Immediate::Lower(Reference(l)) => resolve(l) << 20 >> 20,
        Immediate::Uequ(s) => *equs.get(s).unwrap() as i32 >> 12,
        Immediate::Lequ(s) => (*equs.get(s).unwrap() as i32) << 20 >> 20,
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
