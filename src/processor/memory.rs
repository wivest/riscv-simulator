use crate::instruction::Instruction;
use std::collections::HashMap;

#[derive(Debug)]
enum Word<I, O> {
    Instruction(Instruction<I, O>),
    Value(u32),
}

#[derive(Debug)]
pub struct Sect<I, O> {
    pub memory: Memory<I, O>,
    pub pc: usize,
}

#[derive(Debug)]
pub struct Memory<I, O> {
    words: HashMap<usize, Word<I, O>>,
}

impl<I: Copy, O: Copy> Memory<I, O> {
    pub fn new() -> Memory<I, O> {
        Memory {
            words: HashMap::new(),
        }
    }

    pub fn get(&self, addr: usize) -> Option<u8> {
        let word = match self.words.get(&(addr / 4))? {
            Word::Instruction(_) => todo!(),
            Word::Value(v) => *v,
        };
        Some(word.to_ne_bytes()[addr % 4]) // TODO: endianness
    }

    pub fn set(&mut self, addr: usize, value: u8) {
        let cell = self.words.get(&(addr / 4)).unwrap_or(&Word::Value(0));
        let word = match cell {
            Word::Instruction(_) => return, // TODO: error
            Word::Value(v) => *v,
        };
        let mut bytes = word.to_ne_bytes();
        bytes[addr % 4] = value;
        self.words
            .insert(addr / 4, Word::Value(u32::from_ne_bytes(bytes)));
    }

    pub fn load_instr(&self, pc: usize) -> Option<Instruction<I, O>> {
        let word = self.words.get(&(pc / 4))?;
        match word {
            Word::Instruction(i) => Some(*i),
            Word::Value(_) => None,
        }
    }

    pub fn store_instr(&mut self, addr: usize, instr: Instruction<I, O>) {
        self.words.insert(addr / 4, Word::Instruction(instr));
    }
}
