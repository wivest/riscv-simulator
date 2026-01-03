use crate::processor::Processor;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SType {
    Sb,
    Sh,
    Sw,
}

impl SType {
    pub fn execute(&self, cpu: &mut Processor, rs1: usize, rs2: usize, imm: i32) {
        match self {
            SType::Sb => {
                let val1 = cpu.get_reg(rs1);
                let val2 = cpu.get_reg(rs2);
                let address = (val1 + imm) as usize;
                cpu.memory.insert(address, val2 as u8);
            }
            SType::Sh => {
                let val1 = cpu.get_reg(rs1);
                let val2 = cpu.get_reg(rs2);
                let address = (val1 + imm) as usize;
                cpu.memory.insert(address, val2 as u8);
                cpu.memory.insert(address + 1, (val2 >> 8) as u8);
            }
            SType::Sw => {
                let val1 = cpu.get_reg(rs1);
                let val2 = cpu.get_reg(rs2);
                let address = (val1 + imm) as usize;
                cpu.memory.insert(address, val2 as u8);
                cpu.memory.insert(address + 1, (val2 >> 8) as u8);
                cpu.memory.insert(address + 2, (val2 >> 16) as u8);
                cpu.memory.insert(address + 3, (val2 >> 24) as u8);
            }
        }
        cpu.pc += 4;
    }
}
