use chumsky::{error::Rich, span::SimpleSpan};

use crate::language::{
    instruction::Instruction::{self, *},
    token::{Definition, Immediate, Offset, Reference},
    word::Word,
};

use crate::parser::section::Section;
use crate::processor::memory::Memory;
use std::collections::HashMap;

pub struct Linker<'src> {
    defs: HashMap<Definition<'src>, u32>,
    memory: HashMap<u32, Word<Immediate<'src>, Offset<'src>>>,
    links: Vec<(u32, u32, String)>,
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

    pub fn link<'a>(self) -> Result<Memory<i32, i32>, Vec<Rich<'a, char>>> {
        let mut oks = HashMap::new();
        let mut errs = Vec::<Rich<'a, char>>::new();

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
        for (at, b, link) in self.links {
            match self.defs.get(&Definition(&link)) {
                Some(&addr) => mem.set(at, addr.to_le_bytes()[b as usize]),
                None => errs.push(Rich::custom(SimpleSpan::from(0..0), format!("{link}"))),
            };
        }
        if errs.is_empty() { Ok(mem) } else { Err(errs) }
    }
}

pub fn link_instr<'a>(
    instr: Instruction<Immediate, Offset>,
    addr: u32,
    defs: &HashMap<Definition, u32>,
    equs: &HashMap<String, u32>,
) -> Result<Instruction<i32, i32>, Rich<'a, char>> {
    let resolve = |l| match defs.get(&Definition(l)) {
        Some(&value) => Ok(value as i32),
        None => return Err(Rich::custom(SimpleSpan::from(0..0), format!("{l}"))),
    };
    let load_const = |s| match equs.get(s) {
        Some(&c) => Ok(c as i32),
        None => Err(Rich::custom(SimpleSpan::from(0..0), format!("{s}"))),
    };
    let resolve_rel = |l| Ok::<i32, Rich<'a, char>>(resolve(l)? - addr as i32);
    let calc_offset = |offset| match offset {
        Offset::Label(Reference(l)) => resolve_rel(l),
        Offset::Value(v) => Ok(v),
    };
    let calc_imm = |imm| match imm {
        Immediate::Value(v) => Ok::<i32, Rich<'a, char>>(v),
        Immediate::Upper(Reference(l)) => Ok(resolve(l)? >> 12),
        Immediate::UpperPseudo(Reference(l)) => Ok((resolve(l)? + 0x800) >> 12),
        Immediate::Lower(Reference(l)) => Ok(resolve(l)? << 20 >> 20),
        Immediate::PcrelHi(Reference(l)) => Ok(resolve_rel(l)? + 0x800 >> 12),
        Immediate::PcrelLo(Reference(l)) => Ok((resolve_rel(l)? << 20 >> 20) + 4), // +4 only comes from call/tail (change?)
        Immediate::EquUpper(s) => Ok((load_const(s)? + 0x800) >> 12),
        Immediate::Equ20(s) => Ok(load_const(s)? << 12 >> 12),
        Immediate::Equ12(s) => Ok(load_const(s)? << 20 >> 20),
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
