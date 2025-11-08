use crate::processor::Processor;

#[derive(Debug, Clone, Copy)]
pub enum SType {
    Sb,
}

impl SType {
    pub fn execute(&self, cpu: &mut Processor, rs1: usize, rs2: usize, imm: i32) {
        match self {
            SType::Sb => {
                let val1 = cpu.registers[rs1];
                let val2 = cpu.registers[rs2];
                let address = (val1 + imm) as usize;
                cpu.memory.insert(address, val2 as u8);
            }
        }
        cpu.set_pc(cpu.get_pc() + 4);
    }
}
