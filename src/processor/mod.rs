use crate::cli::command::Executable;
use crate::language::instruction::Instruction;
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

    pub fn execute(&mut self, command: Executable) {
        match command {
            Executable::Step(n) => {
                for i in 0..n {
                    let instr = self.step();
                    if let Some(instr) = instr {
                        println!("[Step {} at {:#010x}]: {instr}", i + 1, self.pc);
                    }
                }
            }
            Executable::Run => self.run(),
            Executable::Output => println!("{}", self.memory.flush()),
            Executable::Memory => println!("{}", self.memory),
            Executable::Registers => println!("{:?}", self.registers),
            Executable::All => {
                println!("{}", self.memory);
                println!("{:?}", self.registers);
            }
        }
    }

    fn step(&mut self) -> Option<Instruction<i32, i32>> {
        let Some(instr) = self.memory.load_instr(self.pc) else {
            return None;
        };
        instr.execute(self);
        Some(instr)
    }

    fn run(&mut self) {
        loop {
            if let Some(instr) = self.step() {
                if let Instruction::Ebreak = instr {
                    return;
                }
            }
        }
    }
}
