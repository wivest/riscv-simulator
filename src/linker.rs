use chumsky::{error::Rich, span::SimpleSpan};

use crate::language::{
    instruction::Instruction::{self, *},
    token::{Immediate, Label, Offset, Reference},
    word::Word,
};

use crate::parser::section::Section;
use crate::processor::memory::Memory;
use std::collections::HashMap;

pub struct Linker<'src> {
    defs: HashMap<Label<'src>, u32>,
    memory: HashMap<u32, Word<Immediate<'src>, Offset<'src>>>,
    links: Vec<(u32, u32, Label<'src>, SimpleSpan)>,
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

    pub fn import_section(
        &mut self,
        sect: Section<'src, Immediate<'src>, Offset<'src>>,
    ) -> Result<(), Vec<Rich<'src, char>>> {
        for (at, word) in sect.content {
            self.memory.insert(sect.base / 4 + at, word);
        }

        let mut errs = Vec::new();
        for (def, at) in sect.defs {
            match self.defs.insert(def.0, sect.base / 4 + at) {
                Some(_) => errs.push(Rich::custom(def.1, format!("duplicate label"))),
                None => (),
            }
        }
        if !errs.is_empty() {
            return Err(errs);
        }

        self.links.extend(sect.links);
        self.equs.extend(sect.equs);
        Ok(())
    }

    pub fn link(self) -> Result<Memory<i32, i32>, Vec<Rich<'src, char>>> {
        let mut oks = HashMap::new();
        let mut errs = Vec::new();

        for (div4, word) in self.memory {
            let word = match word {
                Word::Instruction(i) => match link_instr(i, div4 * 4, &self.defs, &self.equs) {
                    Ok(ins) => Word::Instruction(ins),
                    Err(e) => {
                        errs.push(e);
                        continue;
                    }
                },
                Word::Value(v) => Word::Value(v),
            };
            oks.insert(div4, word);
        }

        let result = if errs.is_empty() { Ok(oks) } else { Err(errs) };

        let mut mem = Memory::from(result?);
        let mut errs = Vec::new();
        for (at, b, link, span) in self.links {
            match self.defs.get(&link) {
                Some(&addr) => mem.set(at, addr.to_le_bytes()[b as usize]),
                None => {
                    if b == 0 {
                        errs.push(Rich::custom(
                            span,
                            format!("unknown label, define \"{link}:\""),
                        ))
                    }
                }
            };
        }
        if errs.is_empty() { Ok(mem) } else { Err(errs) }
    }
}

pub fn link_instr<'src>(
    instr: Instruction<Immediate, Offset>,
    addr: u32,
    defs: &HashMap<Label<'src>, u32>,
    equs: &HashMap<String, u32>,
) -> Result<Instruction<i32, i32>, Rich<'src, char>> {
    let resolve = |l, span| match defs.get(&l) {
        Some(&value) => Ok(value as i32),
        None => Err(Rich::custom(
            span,
            format!("unknown label, define \"{l}:\""),
        )),
    };
    let load_const = |s, span| match equs.get(s) {
        Some(&c) => Ok(c as i32),
        None => Err(Rich::custom(
            span,
            format!("unknown identifier, define \".equ {s}, <value>\""),
        )),
    };
    let resolve_rel = |l, span| Ok::<i32, Rich<'src, char>>(resolve(l, span)? - addr as i32);

    let calc_offset = |offset| match offset {
        Offset::Label(Reference(l, span)) => resolve_rel(l, span),
        Offset::Value(v) => Ok(v),
    };
    let calc_imm = |imm| match imm {
        Immediate::Value(v) => Ok::<i32, Rich<'src, char>>(v),
        Immediate::Upper(Reference(l, s)) => Ok(resolve(l, s)? >> 12),
        Immediate::UpperPseudo(Reference(l, s)) => Ok((resolve(l, s)? + 0x800) >> 12),
        Immediate::Lower(Reference(l, s)) => Ok(resolve(l, s)? << 20 >> 20),
        Immediate::PcrelHi(Reference(l, s)) => Ok(resolve_rel(l, s)? + 0x800 >> 12),
        Immediate::PcrelLo(Reference(l, s)) => Ok((resolve_rel(l, s)? << 20 >> 20) + 4), // +4 only comes from call/tail (change?)
        Immediate::EquUpper(Reference(Label(l), s)) => Ok((load_const(l, s)? + 0x800) >> 12),
        Immediate::Equ20(Reference(Label(l), s)) => Ok(load_const(l, s)? << 12 >> 12),
        Immediate::Equ12(Reference(Label(l), s)) => Ok(load_const(l, s)? << 20 >> 20),
    };

    Ok(match instr {
        BType {
            name,
            rs1,
            rs2,
            offset,
        } => BType {
            name,
            rs1,
            rs2,
            offset: calc_offset(offset)?,
        },
        IType { name, rd, rs, imm } => IType {
            name,
            rd,
            rs,
            imm: calc_imm(imm)?,
        },
        JType { name, rd, imm } => JType {
            name,
            rd,
            imm: calc_offset(imm)?,
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
            imm: calc_imm(imm)?,
        },
        UType { name, rd, imm } => UType {
            name,
            rd,
            imm: calc_imm(imm)?,
        },
        Ebreak => Ebreak,
    })
}
