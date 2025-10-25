use crate::processor::Processor;

#[derive(Debug, Clone, Copy)]
pub enum SType {
    Sb,
}

impl SType {
    pub fn execute(&self, cpu: &mut Processor, rs1: i32, rs2: i32, imm: i32) {
        match self {
            SType::Sb => {
                let val1 = cpu.registers[rs1 as usize];
                let val2 = cpu.registers[rs2 as usize];
                let address = (val1 + imm) as usize;
                cpu.memory.insert(address, val2 as u8);
            }
        }
        cpu.pc += 4;
    }
}
