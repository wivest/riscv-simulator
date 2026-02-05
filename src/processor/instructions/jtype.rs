use super::JType;
use crate::processor::Processor;

impl JType {
    pub fn execute(&self, cpu: &mut Processor, rd: usize, imm: i32) {
        match self {
            JType::Jal => {
                cpu.set_reg(rd, cpu.pc as i32 + 4);
                let pc = cpu.pc as i32;
                cpu.pc = (pc as i32 + imm) as usize;
            }
        }
        cpu.pc += 4;
    }
}
