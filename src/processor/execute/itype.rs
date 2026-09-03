use super::IType;
use crate::processor::Processor;

impl IType {
    pub fn execute(&self, cpu: &mut Processor, rd: u32, rs: u32, imm: i32) {
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
                let addr = src + imm;
                let byte0 = cpu.memory.get(addr as u32).unwrap_or(0) as u32;
                let byte1 = cpu.memory.get(addr as u32 + 1).unwrap_or(0) as u32;
                let byte2 = cpu.memory.get(addr as u32 + 2).unwrap_or(0) as u32;
                let byte3 = cpu.memory.get(addr as u32 + 3).unwrap_or(0) as u32;
                let word = (byte3 << 24) + (byte2 << 16) + (byte1 << 8) + byte0;
                cpu.set_reg(rd, word as i32);
            }
            IType::Lh => {
                let addr = src + imm;
                let low = cpu.memory.get(addr as u32).unwrap_or(0) as u32;
                let high = cpu.memory.get(addr as u32 + 1).unwrap_or(0) as u32;
                cpu.set_reg(rd, ((high << 8) + low) as i32);
            }
            IType::Lhu => {
                let addr = src + imm;
                let low = cpu.memory.get(addr as u32).unwrap_or(0);
                let high = cpu.memory.get(addr as u32 + 1).unwrap_or(0);
                cpu.set_reg(rd, ((high as u32) << 8 + (low as u32)) as i32);
            }
            IType::Lb => {
                let addr = src + imm;
                let byte = cpu.memory.get(addr as u32).unwrap_or(0);
                cpu.set_reg(rd, byte as i8 as i32);
            }
            IType::Lbu => {
                let addr = src + imm;
                let byte = cpu.memory.get(addr as u32).unwrap_or(0);
                cpu.set_reg(rd, byte as i32);
            }
            // jump
            IType::Jalr => {
                cpu.set_reg(rd, cpu.pc as i32 + 4);
                cpu.pc = (src + imm) as u32;
                return;
            }
        }
        cpu.pc += 4;
    }

    pub fn opcode(&self) -> (u32, u32) {
        match self {
            // arithmetic
            IType::Addi => (0b0010011, 0x0),
            // bitwise logic
            IType::Andi => (0b0010011, 0x7),
            IType::Ori => (0b0010011, 0x6),
            IType::Xori => (0b0010011, 0x4),
            // shift
            IType::Slli => (0b0010011, 0x1),
            IType::Srli => (0b0010011, 0x5),
            IType::Srai => (0b0010011, 0x5), // ignoring immediate
            // load
            IType::Lw => (0b0000011, 0x2),
            IType::Lh => (0b0000011, 0x1),
            IType::Lhu => (0b0000011, 0x5),
            IType::Lb => (0b0000011, 0x0),
            IType::Lbu => (0b0000011, 0x4),
            // jump
            IType::Jalr => (0b1100111, 0x0),
        }
    }
}
