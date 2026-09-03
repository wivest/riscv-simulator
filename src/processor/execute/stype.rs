use super::SType;
use crate::processor::Processor;

impl SType {
    pub fn execute(&self, cpu: &mut Processor, rs1: u32, rs2: u32, imm: i32) {
        match self {
            SType::Sb => {
                let val1 = cpu.get_reg(rs1);
                let val2 = cpu.get_reg(rs2);
                let address = (val1 + imm) as u32;
                cpu.memory.set(address, val2 as u8);
            }
            SType::Sh => {
                let val1 = cpu.get_reg(rs1);
                let val2 = cpu.get_reg(rs2);
                let address = (val1 + imm) as u32;
                cpu.memory.set(address, val2 as u8);
                cpu.memory.set(address + 1, (val2 >> 8) as u8);
            }
            SType::Sw => {
                let val1 = cpu.get_reg(rs1);
                let val2 = cpu.get_reg(rs2);
                let address = (val1 + imm) as u32;
                cpu.memory.set(address, val2 as u8);
                cpu.memory.set(address + 1, (val2 >> 8) as u8);
                cpu.memory.set(address + 2, (val2 >> 16) as u8);
                cpu.memory.set(address + 3, (val2 >> 24) as u8);
            }
        }
        cpu.pc += 4;
    }

    pub fn opcode(&self) -> (u32, u32) {
        match self {
            SType::Sb => (0b0100011, 0x0),
            SType::Sh => (0b0100011, 0x1),
            SType::Sw => (0b0100011, 0x2),
        }
    }
}
