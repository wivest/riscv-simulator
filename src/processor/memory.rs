use crate::{instruction::Instruction, parser::token::Definition};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Word<I, O> {
    Instruction(Instruction<I, O>),
    Value(u32),
}

#[derive(Debug)]
pub struct Sect<I, O> {
    pub base: usize,
    pub pc: usize,
    pub memory: Memory<I, O>,
}

impl<I: Copy, O: Copy> Sect<I, O> {
    pub fn new(base: usize) -> Self {
        Sect {
            base,
            pc: 0,
            memory: Memory::new(),
        }
    }
}

#[derive(Debug)]
pub struct Memory<I, O>(HashMap<usize, Word<I, O>>);

impl<I: Copy, O: Copy> Memory<I, O> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, addr: usize) -> Option<u8> {
        let word = match self.0.get(&(addr / 4))? {
            Word::Instruction(_) => todo!(),
            Word::Value(v) => *v,
        };
        Some(word.to_ne_bytes()[addr % 4]) // TODO: endianness
    }

    pub fn set(&mut self, addr: usize, value: u8) {
        let cell = self.0.get(&(addr / 4)).unwrap_or(&Word::Value(0));
        let word = match cell {
            Word::Instruction(_) => return, // TODO: error
            Word::Value(v) => *v,
        };
        let mut bytes = word.to_ne_bytes();
        bytes[addr % 4] = value;
        self.0
            .insert(addr / 4, Word::Value(u32::from_ne_bytes(bytes)));
    }

    pub fn load_instr(&self, pc: usize) -> Option<Instruction<I, O>> {
        let word = self.0.get(&(pc / 4))?;
        match word {
            Word::Instruction(i) => Some(*i),
            Word::Value(_) => None,
        }
    }

    pub fn store_instr(&mut self, addr: usize, instr: Instruction<I, O>) {
        self.0.insert(addr / 4, Word::Instruction(instr));
    }

    pub fn copy_memory(&mut self, other: Self) {
        self.0.extend(other.0);
    }

    pub fn link<
        'a,
        F: Fn(Instruction<I, O>, usize, &'a HashMap<Definition<'a>, usize>) -> Instruction<i32, i32>,
    >(
        self,
        tr: F,
        defs: &'a HashMap<Definition<'a>, usize>,
        base: usize,
    ) -> Memory<i32, i32> {
        Memory(
            self.0
                .into_iter()
                .map(|(div4, word)| {
                    let word = match word {
                        Word::Instruction(i) => Word::Instruction(tr(i, base + div4 * 4, defs)),
                        Word::Value(v) => Word::Value(v),
                    };
                    (base / 4 + div4, word)
                })
                .collect(),
        )
    }
}

impl<I: std::fmt::Debug, O: std::fmt::Debug> std::fmt::Display for Memory<I, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut words = self.0.iter().collect::<Vec<_>>();
        words.sort_by_key(|&(k, _)| *k);
        for (div4, word) in words {
            writeln!(f, "{:#x}: {:?}", div4 * 4, word)?
        }
        std::fmt::Result::Ok(())
    }
}
