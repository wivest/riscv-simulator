use crate::processor::Processor;

#[derive(Debug, Clone, Copy)]
pub enum UType {
    Li,
    Lui,
    Auipc,
}

impl UType {
    pub fn execute(&self, cpu: &mut Processor, rd: usize, imm: i32) {
        match self {
            UType::Li => cpu.set_reg(rd, imm),
            UType::Lui => cpu.set_reg(rd, imm << 12),
            UType::Auipc => cpu.set_reg(rd, cpu.pc as i32 + imm << 12),
        }
        cpu.pc += 4;
    }
}
