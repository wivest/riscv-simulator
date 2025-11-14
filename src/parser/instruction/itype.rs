use crate::processor::Processor;

#[derive(Debug, Clone, Copy)]
pub enum IType {
    Addi,
    Lb,
}

impl IType {
    pub fn execute(&self, cpu: &mut Processor, rd: usize, rs: usize, imm: i32) {
        match self {
            IType::Addi => cpu.set_reg(rd, cpu.get_reg(rs) + imm),
            IType::Lb => {
                let address = cpu.get_reg(rs);
                let byte = *cpu.memory.get(&(address as usize)).unwrap_or(&0);
                cpu.set_reg(rd, byte as i32);
            }
        }
        cpu.pc += 4;
    }
}
