use super::BType;
use crate::processor::Processor;

impl BType {
    pub fn execute(&self, cpu: &mut Processor, rs1: u32, rs2: u32, offset: i32) {
        let left = cpu.get_reg(rs1);
        let right = cpu.get_reg(rs2);

        match self {
            BType::Beq => {
                if left == right {
                    cpu.pc = (cpu.pc as i32 + offset) as u32;
                } else {
                    cpu.pc += 4;
                }
            }
            BType::Bne => {
                if left != right {
                    cpu.pc = (cpu.pc as i32 + offset) as u32;
                } else {
                    cpu.pc += 4;
                }
            }
            BType::Blt => {
                if left < right {
                    cpu.pc = (cpu.pc as i32 + offset) as u32;
                } else {
                    cpu.pc += 4;
                }
            }
            BType::Bltu => {
                if (left as u32) < (right as u32) {
                    cpu.pc = (cpu.pc as i32 + offset) as u32;
                } else {
                    cpu.pc += 4;
                }
            }
            BType::Bge => {
                if left >= right {
                    cpu.pc = (cpu.pc as i32 + offset) as u32;
                } else {
                    cpu.pc += 4;
                }
            }
            BType::Bgeu => {
                if (left as u32) >= (right as u32) {
                    cpu.pc = (cpu.pc as i32 + offset) as u32;
                } else {
                    cpu.pc += 4;
                }
            }
        }
    }
}
