use std::collections::HashMap;

#[derive(Debug)]
enum Cell {
    Instruction,
    Value(u32),
}

#[derive(Debug)]
pub struct Memory {
    cells: HashMap<usize, Cell>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            cells: HashMap::new(),
        }
    }

    pub fn get(&self, addr: usize) -> Option<u8> {
        let word = match self.cells.get(&(addr / 4))? {
            Cell::Instruction => todo!(),
            Cell::Value(v) => *v,
        };
        Some(word.to_ne_bytes()[addr % 4]) // TODO: endianness
    }

    pub fn set(&mut self, addr: usize, value: u8) {
        let cell = self.cells.get(&(addr / 4)).unwrap_or(&Cell::Value(0));
        let word = match cell {
            Cell::Instruction => return, // TODO: error
            Cell::Value(v) => *v,
        };
        let mut bytes = word.to_ne_bytes();
        bytes[addr % 4] = value;
        self.cells
            .insert(addr / 4, Cell::Value(u32::from_ne_bytes(bytes)));
    }
}
