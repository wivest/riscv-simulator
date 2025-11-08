use crate::processor::Processor;

#[derive(Debug, Clone, Copy)]
pub enum BType {
    Beq,
}

impl BType {
    pub fn execute(&self, cpu: &mut Processor, rs1: usize, rs2: usize, offset: i32) {
        match self {
            BType::Beq => {
                let left = cpu.registers[rs1];
                let right = cpu.registers[rs2];
                if left == right {
                    let pc = cpu.get_pc() as i32;
                    cpu.set_pc((pc + offset) as usize);
                } else {
                    cpu.set_pc(cpu.get_pc() + 4);
                }
            }
        }
    }
}
