use crate::instruction::Instruction;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Word<I, O> {
    Instruction(Instruction<I, O>),
    Value(u32),
}

#[derive(Debug)]
pub struct Memory<I, O>(HashMap<usize, Word<I, O>>);

impl<I: Copy, O: Copy> Memory<I, O> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn from(content: HashMap<usize, Word<I, O>>) -> Self {
        Self(content)
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
