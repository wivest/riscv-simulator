use crate::processor::Processor;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BType {
    Beq,
    Bne,
}

impl BType {
    pub fn execute(&self, cpu: &mut Processor, rs1: usize, rs2: usize, offset: i32) {
        match self {
            BType::Beq => {
                let left = cpu.get_reg(rs1);
                let right = cpu.get_reg(rs2);
                if left == right {
                    cpu.pc = (cpu.pc as i32 + offset) as usize;
                } else {
                    cpu.pc += 4;
                }
            }
            BType::Bne => {
                let left = cpu.get_reg(rs1);
                let right = cpu.get_reg(rs2);
                if left != right {
                    cpu.pc = (cpu.pc as i32 + offset) as usize;
                } else {
                    cpu.pc += 4;
                }
            }
        }
    }
}
