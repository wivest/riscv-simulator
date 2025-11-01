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
            RType::Add => {
                cpu.registers[rd as usize] =
                    cpu.registers[rs1 as usize] + cpu.registers[rs2 as usize];
            }
            RType::Sub => {
                cpu.registers[rd as usize] =
                    cpu.registers[rs1 as usize] - cpu.registers[rs2 as usize];
            }
            RType::Mul => {
                cpu.registers[rd as usize] =
                    cpu.registers[rs1 as usize] * cpu.registers[rs2 as usize];
            }
            RType::Div => {
                cpu.registers[rd as usize] =
                    cpu.registers[rs1 as usize] / cpu.registers[rs2 as usize];
            }
            RType::Rem => {
                cpu.registers[rd as usize] =
                    cpu.registers[rs1 as usize] % cpu.registers[rs2 as usize];
            }
        }
        cpu.pc += 4;
    }
}
