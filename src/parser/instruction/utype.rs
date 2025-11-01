use crate::processor::Processor;

#[derive(Debug, Clone, Copy)]
pub enum UType {
    Li,
}

impl UType {
    pub fn execute(&self, cpu: &mut Processor, rd: usize, imm: i32) {
        match self {
            UType::Li => {
                cpu.registers[rd] = imm;
            }
        }
        cpu.pc += 4;
    }
}
