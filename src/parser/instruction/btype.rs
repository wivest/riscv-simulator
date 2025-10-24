use crate::processor::Processor;

#[derive(Debug, Clone, Copy)]
pub enum BType {
    Beq,
}

impl BType {
    pub fn execute(&self, cpu: &mut Processor, rs1: i32, rs2: i32, offset: i32) {
        match self {
            BType::Beq => {
                let left = cpu.registers[rs1 as usize];
                let right = cpu.registers[rs2 as usize];
                if left == right {
                    let pc = cpu.pc as i32;
                    cpu.pc = (pc + offset) as usize;
                } else {
                    cpu.pc += 4;
                }
            }
        }
    }
}
