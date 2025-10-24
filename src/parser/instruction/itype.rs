use crate::processor::Processor;

#[derive(Debug, Clone, Copy)]
pub enum IType {
    Addi,
}
impl IType {
    pub fn execute(&self, cpu: &mut Processor, rd: i32, rs: i32, imm: i32) {
        match self {
            IType::Addi => {
                cpu.registers[rd as usize] = cpu.registers[rs as usize] + imm;
            }
        }
        cpu.pc += 4;
    }
}
