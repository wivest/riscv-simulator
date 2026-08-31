use crate::language::{instruction::Instruction, word::Word};

use std::collections::HashMap;

const PRINT_BASE: usize = 0xffffc000;
const STATUS_OFFSET: usize = 0x0008;
const DATA_OFFSET: usize = 0x000c;

#[derive(Debug)]
pub struct Memory<I, O>(HashMap<usize, Word<I, O>>);

impl<I: Copy, O: Copy> Memory<I, O> {
    pub fn from(content: HashMap<usize, Word<I, O>>) -> Self {
        let mut result = Self(content);
        // set status bit to ready
        result.set(PRINT_BASE + STATUS_OFFSET, 1);
        result
    }

    pub fn get(&self, addr: usize) -> Option<u8> {
        let word = match self.0.get(&(addr / 4))? {
            Word::Instruction(_) => todo!(),
            Word::Value(v) => *v,
        };
        Some(word.to_le_bytes()[addr % 4])
    }

    fn mmio(&mut self, addr: usize, value: u8) -> bool {
        if addr != PRINT_BASE + DATA_OFFSET {
            false
        } else {
            // setting status bit has no sense, because we have a single thread
            // we leave it for correctness
            self.set(PRINT_BASE + STATUS_OFFSET, 0);
            print!("{}", value as char);
            self.set(PRINT_BASE + STATUS_OFFSET, 1);
            true
        }
    }

    pub fn set(&mut self, addr: usize, value: u8) {
        if self.mmio(addr, value) {
            return;
        }

        let cell = self.0.get(&(addr / 4)).unwrap_or(&Word::Value(0));
        let word = match cell {
            Word::Instruction(_) => return, // TODO: error
            Word::Value(v) => *v,
        };
        let mut bytes = word.to_ne_bytes();
        bytes[addr % 4] = value;
        self.0
            .insert(addr / 4, Word::Value(u32::from_le_bytes(bytes)));
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
