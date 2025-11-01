use crate::processor::Processor;

#[derive(Debug, Clone, Copy)]
pub enum IType {
    Addi,
    Lb,
}
impl IType {
    pub fn execute(&self, cpu: &mut Processor, rd: usize, rs: usize, imm: i32) {
        match self {
            IType::Addi => cpu.registers[rd as usize] = cpu.registers[rs as usize] + imm,
            IType::Lb => {
                let address = cpu.registers[rs as usize];
                let byte = *cpu.memory.get(&(address as usize)).unwrap_or(&0);
                cpu.registers[rd as usize] = byte as i32;
            }
        }
        cpu.pc += 4;
    }
}
