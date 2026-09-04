use crate::cli::command::Executable;
use crate::language::instruction::Instruction;
use memory::Memory;
use terminal_size::{Width, terminal_size};

pub mod execute;
pub mod memory;

pub struct Processor {
    pub pc: u32,
    registers: [i32; 32],
    memory: Memory<i32, i32>,
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
            Executable::Memory => print!("{}", self.memory),
            Executable::Registers => println!("{}", self.fmt_reg()),
            Executable::Instructions => print!("{}", self.memory.list_instr()),
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

    fn fmt_reg(&self) -> String {
        const SEP: usize = 3;
        const ENTRY: usize = 23;
        const NAMES: [&str; 32] = [
            "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0/fp", "s1", "a0", "a1", "a2",
            "a3", "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10",
            "s11", "t3", "t4", "t5", "t6",
        ];

        let width = terminal_size().map(|(Width(w), _)| w).unwrap_or(80) as usize;
        let columns = ((width + SEP) / (ENTRY + SEP)).clamp(1, 4);

        let mut acc = String::new();
        acc += &format!("pc: {:#010x}\n", self.pc);
        for (i, val) in self.registers.iter().enumerate() {
            acc += &format!("x{i:<2} {:<7}: {val:#010x}", format!("({})", NAMES[i]));
            match (i + 1) % columns {
                0 => acc += "\n",
                _ => acc += " | ",
            }
        }
        if acc.ends_with("\n") {
            acc.pop();
        }
        acc
    }
}
