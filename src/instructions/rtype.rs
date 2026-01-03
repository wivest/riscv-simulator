use crate::processor::Processor;

#[derive(Debug, Clone, Copy, PartialEq)]
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
            RType::Add => cpu.set_reg(rd, cpu.get_reg(rs1) + cpu.get_reg(rs2)),
            RType::Sub => cpu.set_reg(rd, cpu.get_reg(rs1) - cpu.get_reg(rs2)),
            RType::Mul => cpu.set_reg(rd, cpu.get_reg(rs1) * cpu.get_reg(rs2)),
            RType::Div => cpu.set_reg(rd, cpu.get_reg(rs1) / cpu.get_reg(rs2)),
            RType::Rem => cpu.set_reg(rd, cpu.get_reg(rs1) % cpu.get_reg(rs2)),
        }
        cpu.pc += 4;
    }
}
