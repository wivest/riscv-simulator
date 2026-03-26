use instructions::Instruction;
use memory::Memory;

pub mod instructions;
pub mod memory;

pub struct Processor {
    pub pc: usize,
    pub memory: Memory,
    registers: [i32; 32],
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            pc: 0,
            registers: [0; 32],
            memory: Memory::new(),
        }
    }

    pub fn get_reg(&self, index: usize) -> i32 {
        if index == 0 { 0 } else { self.registers[index] }
    }

    pub fn set_reg(&mut self, index: usize, value: i32) {
        if index != 0 {
            self.registers[index] = value;
        };
    }

    pub fn store_instrs(&mut self, instrs: Vec<Instruction>, offset: usize) {
        for (i, instr) in instrs.into_iter().enumerate() {
            self.memory.store_instr(offset + i * 4, instr);
        }
    }

    pub fn execute(&mut self, offset: usize) {
        self.pc = offset;
        loop {
            let Some(instr) = self.memory.load_instr(self.pc) else {
                break;
            };
            println!("{instr:?}");
            if !instr.execute(self) {
                break;
            }
        }
    }
}
