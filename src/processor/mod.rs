use memory::Memory;

pub mod execute;
pub mod memory;

pub struct Processor {
    pub pc: u32,
    pub memory: Memory<i32, i32>,
    registers: [i32; 32],
}

impl Processor {
    pub fn new(reset: u32, memory: Memory<i32, i32>) -> Self {
        Processor {
            pc: reset,
            registers: [0; 32],
            memory,
        }
    }

    pub fn get_reg(&self, index: u32) -> i32 {
        if index == 0 {
            0
        } else {
            self.registers[index as usize]
        }
    }

    pub fn set_reg(&mut self, index: u32, value: i32) {
        if index != 0 {
            self.registers[index as usize] = value;
        };
    }

    pub fn execute(&mut self) {
        loop {
            let Some(instr) = self.memory.load_instr(self.pc) else {
                break;
            };
            println!("[PC {:#010x}]: {instr}", self.pc);
            if !instr.execute(self) {
                break;
            }
        }
    }
}
