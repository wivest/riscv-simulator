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

    pub fn opcode(&self) -> (u32, u32) {
        match self {
            BType::Beq => (0b1100011, 0x0),
            BType::Bne => (0b1100011, 0x1),
            BType::Blt => (0b1100011, 0x4),
            BType::Bltu => (0b1100011, 0x6),
            BType::Bge => (0b1100011, 0x5),
            BType::Bgeu => (0b1100011, 0x7),
        }
    }
}
