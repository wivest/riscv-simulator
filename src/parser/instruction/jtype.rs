use crate::processor::Processor;

#[derive(Debug, Clone, Copy)]
pub enum JType {
    Jal,
}

impl JType {
    pub fn execute(&self, cpu: &mut Processor, rd: usize, imm: i32) {
        match self {
            JType::Jal => {
                cpu.registers[rd] = cpu.pc as i32 + 4;
                let pc = cpu.pc as i32;
                cpu.pc = (pc + imm) as usize;
            }
        }
        cpu.pc += 4;
    }
}
