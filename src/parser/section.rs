use crate::language::{instruction::Instruction, token::Definition, word::Word};

use std::collections::HashMap;

#[derive(Debug)]
pub struct Section<'src, I, O> {
    pub base: usize,
    pub pc: usize,
    pub defs: HashMap<Definition<'src>, usize>,
    pub content: HashMap<usize, Word<I, O>>,
    pub links: Vec<(usize, usize, String)>,
    pub equs: HashMap<String, u32>,
}

impl<'src, I: Copy, O: Copy> Section<'src, I, O> {
    pub fn new(base: usize, pc: usize) -> Self {
        Section {
            base,
            pc,
            defs: HashMap::new(),
            content: HashMap::new(),
            links: Vec::new(),
            equs: HashMap::new(),
        }
    }

    pub fn set(&mut self, addr: usize, value: u8) {
        let addr = self.base + addr;
        let cell = self.content.get(&(addr / 4)).unwrap_or(&Word::Value(0));
        let word = match cell {
            Word::Instruction(_) => return, // TODO: error
            Word::Value(v) => *v,
        };
        let mut bytes = word.to_ne_bytes();
        bytes[addr % 4] = value;
        self.content
            .insert(addr / 4, Word::Value(u32::from_ne_bytes(bytes)));
    }

    pub fn store_instr(&mut self, addr: usize, instr: Instruction<I, O>) {
        self.content
            .insert((self.base + addr) / 4, Word::Instruction(instr));
    }
}
