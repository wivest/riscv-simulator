use crate::processor::Processor;

#[derive(Debug, Clone, Copy)]
pub enum RType {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

impl RType {
    pub fn execute(&self, cpu: &mut Processor, rd: usize, rs1: usize, rs2: usize) {
        match self {
            RType::Add => cpu.registers[rd] = cpu.registers[rs1] + cpu.registers[rs2],
            RType::Sub => cpu.registers[rd] = cpu.registers[rs1] - cpu.registers[rs2],
            RType::Mul => cpu.registers[rd] = cpu.registers[rs1] * cpu.registers[rs2],
            RType::Div => cpu.registers[rd] = cpu.registers[rs1] / cpu.registers[rs2],
            RType::Rem => cpu.registers[rd] = cpu.registers[rs1] % cpu.registers[rs2],
        }
        cpu.pc += 4;
    }
}
