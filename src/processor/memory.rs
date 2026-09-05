use crate::language::{instruction::Instruction, word::Word};
use std::collections::HashMap;
use terminal_size::{Width, terminal_size};

const PRINT_BASE: u32 = 0xffffc000;
const STATUS_OFFSET: u32 = 0x0008;
const DATA_OFFSET: u32 = 0x000c;

#[derive(Debug)]
pub struct Memory<I, O> {
    content: HashMap<u32, Word<I, O>>,
    output: Vec<char>,
}

impl<I: Copy, O: Copy> Memory<I, O> {
    pub fn from(content: HashMap<u32, Word<I, O>>) -> Self {
        let mut result = Self {
            content,
            output: Vec::new(),
        };
        // set status bit to ready
        result.set(PRINT_BASE + STATUS_OFFSET, 1);
        result
    }

    fn mmio(&mut self, addr: u32, value: u8) -> bool {
        if addr != PRINT_BASE + DATA_OFFSET {
            false
        } else {
            // setting status bit has no sense, because we have a single thread
            // we leave it for correctness
            self.set(PRINT_BASE + STATUS_OFFSET, 0);
            self.output.push(value as char);
            self.set(PRINT_BASE + STATUS_OFFSET, 1);
            true
        }
    }

    pub fn set(&mut self, addr: u32, value: u8) {
        if self.mmio(addr, value) {
            return;
        }

        let cell = self.content.get(&(addr / 4)).unwrap_or(&Word::Value(0));
        let word = match cell {
            Word::Instruction(_) => return, // TODO: error
            Word::Value(v) => *v,
        };
        let mut bytes = word.to_ne_bytes();
        bytes[addr as usize % 4] = value;
        self.content
            .insert(addr / 4, Word::Value(u32::from_le_bytes(bytes)));
    }

    pub fn load_instr(&self, pc: u32) -> Option<Instruction<I, O>> {
        let word = self.content.get(&(pc / 4))?;
        match word {
            Word::Instruction(i) => Some(*i),
            Word::Value(_) => None,
        }
    }

    pub fn flush(&self) -> String {
        self.output.iter().collect()
    }
}

impl Memory<i32, i32> {
    pub fn get(&self, addr: u32) -> Option<u8> {
        let word = match self.content.get(&(addr / 4))? {
            Word::Instruction(instr) => instr.encode(),
            Word::Value(v) => *v,
        };
        Some(word.to_le_bytes()[addr as usize % 4])
    }

    pub fn word(&self, addr: u32) -> &Word<i32, i32> {
        self.content.get(&(addr / 4)).unwrap_or(&Word::Value(0))
    }

    pub fn list_instr(&self) -> String {
        let mut words = self
            .content
            .iter()
            .filter(|&(_, word)| match word {
                Word::Instruction(_) => true,
                _ => false,
            })
            .collect::<Vec<_>>();
        words.sort_by_key(|&(k, _)| *k);
        let mut acc = String::new();
        for (div4, word) in words {
            acc += &format!("{:#010x}:\t{word}\n", div4 * 4);
        }
        acc
    }
}

impl std::fmt::Display for Memory<i32, i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const ADDR_FMT: usize = "0x12345678".len() + ":     ".len() + "  ".len();
        const SEP: usize = "  ".len();
        const ENTRY: usize = "00 00 00 00".len() + "....".len();

        let width = terminal_size().map(|(Width(w), _)| w).unwrap_or(80) as usize;
        let cols = ((width - ADDR_FMT + SEP) / (ENTRY + SEP)).clamp(1, 4) as u32;

        let mut keys: Vec<u32> = self.content.keys().copied().collect();
        keys.sort();
        let rows = keys.chunk_by(|a, b| a / cols == b / cols);

        for row in rows {
            let base = row[0] - row[0] % cols;
            write!(f, "{:#010x}:     ", base * 4)?;

            let mut ascii = String::new();
            for i in 0..cols {
                let word = self.content.get(&(base + i)).unwrap_or(&Word::Value(0));
                write!(f, "{}  ", word.split())?;
                ascii += &word.ascii();
            }
            write!(f, "{}", ascii)?;

            writeln!(f)?;
        }

        std::fmt::Result::Ok(())
    }
}
