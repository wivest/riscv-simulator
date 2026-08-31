use super::IType;
use crate::processor::Processor;

impl IType {
    pub fn execute(&self, cpu: &mut Processor, rd: usize, rs: usize, imm: i32) {
        let src = cpu.get_reg(rs);
        match self {
            // arithmetic
            IType::Addi => cpu.set_reg(rd, src + imm),
            // bitwise logic
            IType::Andi => cpu.set_reg(rd, src & imm),
            IType::Ori => cpu.set_reg(rd, src | imm),
            IType::Xori => cpu.set_reg(rd, src ^ imm),
            // shift
            IType::Slli => cpu.set_reg(rd, src << imm),
            IType::Srli => cpu.set_reg(rd, ((src as u32) >> (imm as u32)) as i32),
            IType::Srai => cpu.set_reg(rd, src >> imm),
            // load
            IType::Lw => {
                let byte0 = cpu.memory.get(src as usize).unwrap_or(0) as u32;
                let byte1 = cpu.memory.get(src as usize + 1).unwrap_or(0) as u32;
                let byte2 = cpu.memory.get(src as usize + 2).unwrap_or(0) as u32;
                let byte3 = cpu.memory.get(src as usize + 3).unwrap_or(0) as u32;
                let word = (byte3 << 24) + (byte2 << 16) + (byte1 << 8) + byte0;
                cpu.set_reg(rd, word as i32);
                println!("lw: {word}, rs: {src}");
            }
            IType::Lh => {
                let low = cpu.memory.get(src as usize).unwrap_or(0) as u32;
                let high = cpu.memory.get(src as usize + 1).unwrap_or(0) as u32;
                cpu.set_reg(rd, ((high << 8) + low) as i32);
            }
            IType::Lhu => {
                let low = cpu.memory.get(src as usize).unwrap_or(0);
                let high = cpu.memory.get(src as usize + 1).unwrap_or(0);
                cpu.set_reg(rd, ((high as u32) << 8 + (low as u32)) as i32);
            }
            IType::Lb => {
                // println!("rs: {src}");
                let byte = cpu.memory.get(src as usize).unwrap_or(0);
                cpu.set_reg(rd, byte as i8 as i32);
            }
            IType::Lbu => {
                let byte = cpu.memory.get(src as usize).unwrap_or(0);
                cpu.set_reg(rd, byte as i32);
            }
            // jump
            IType::Jalr => {
                cpu.set_reg(rd, cpu.pc as i32 + 4);
                cpu.pc = (src + imm) as usize;
                return;
            }
        }
        cpu.pc += 4;
    }
}
