use crate::processor::Processor;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IType {
    Addi,
    Lb,
    Lh,
    Lw,
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
            IType::Lh => {
                let address = cpu.get_reg(rs);
                let low = *cpu.memory.get(&(address as usize)).unwrap_or(&0);
                let high = *cpu.memory.get(&(address as usize + 1)).unwrap_or(&0);
                cpu.set_reg(rd, (high as i32) << 8 + low as i32);
            }
            IType::Lw => {
                let address = cpu.get_reg(rs);
                let byte0 = *cpu.memory.get(&(address as usize)).unwrap_or(&0) as i32;
                let byte1 = *cpu.memory.get(&(address as usize + 1)).unwrap_or(&0) as i32;
                let byte2 = *cpu.memory.get(&(address as usize + 2)).unwrap_or(&0) as i32;
                let byte3 = *cpu.memory.get(&(address as usize + 3)).unwrap_or(&0) as i32;
                let word = byte3 << 24 + byte2 << 16 + byte1 << 8 + byte0;
                cpu.set_reg(rd, word);
            }
        }
        cpu.pc += 4;
    }
}
