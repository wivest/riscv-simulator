use super::JType;
use crate::processor::Processor;

impl JType {
    pub fn execute(&self, cpu: &mut Processor, rd: u32, imm: i32) {
        match self {
            JType::Jal => {
                cpu.set_reg(rd, cpu.pc as i32 + 4);
                cpu.pc = (cpu.pc as i32 + imm) as u32;
            }
        }
    }
}
