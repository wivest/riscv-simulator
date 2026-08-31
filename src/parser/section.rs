use crate::language::{instruction::Instruction, token::Definition, word::Word};

use std::collections::HashMap;

#[derive(Debug)]
pub struct Section<'src, I, O> {
    pub base: u32,
    pub pc: u32,
    pub defs: HashMap<Definition<'src>, u32>,
    pub content: HashMap<u32, Word<I, O>>,
    pub links: Vec<(u32, u32, String)>,
    pub equs: HashMap<String, u32>,
}

impl<'src, I: Copy, O: Copy> Section<'src, I, O> {
    pub fn new(base: u32, pc: u32) -> Self {
        Section {
            base,
            pc,
            defs: HashMap::new(),
            content: HashMap::new(),
            links: Vec::new(),
            equs: HashMap::new(),
        }
    }

    pub fn set(&mut self, addr: u32, value: u8) {
        let addr = self.base + addr;
        let cell = self.content.get(&(addr / 4)).unwrap_or(&Word::Value(0));
        let word = match cell {
            Word::Instruction(_) => return, // TODO: error
            Word::Value(v) => *v,
        };
        let mut bytes = word.to_ne_bytes();
        bytes[addr as usize % 4] = value;
        self.content
            .insert(addr / 4, Word::Value(u32::from_ne_bytes(bytes)));
    }

    pub fn store_instr(&mut self, addr: u32, instr: Instruction<I, O>) {
        self.content
            .insert((self.base + addr) / 4, Word::Instruction(instr));
    }
}
