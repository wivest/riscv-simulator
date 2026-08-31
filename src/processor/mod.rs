use memory::Memory;

pub mod execute;
pub mod memory;

pub struct Processor {
    pub pc: usize,
    pub memory: Memory<i32, i32>,
    pub registers: [i32; 32], // TODO: remove pub
}

impl Processor {
    pub fn new(reset: usize, memory: Memory<i32, i32>) -> Self {
        Processor {
            pc: reset,
            registers: [0; 32],
            memory,
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

    pub fn execute(&mut self) {
        loop {
            let Some(instr) = self.memory.load_instr(self.pc) else {
                println!("{}: not an instr", self.pc);
                break;
            };
            println!("{:#x}: {instr:?} | {}", self.pc, self.registers[10]);
            if !instr.execute(self) {
                break;
            }
        }
    }
}
