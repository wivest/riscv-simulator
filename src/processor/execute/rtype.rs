use super::RType;
use crate::processor::Processor;

impl RType {
    pub fn execute(&self, cpu: &mut Processor, rd: u32, rs1: u32, rs2: u32) {
        let s1 = cpu.get_reg(rs1);
        let s2 = cpu.get_reg(rs2);
        match self {
            // arithmetic
            RType::Add => cpu.set_reg(rd, s1 + s2),
            RType::Sub => cpu.set_reg(rd, s1 - s2),
            RType::Mul => cpu.set_reg(rd, s1 * s2),
            RType::Mulh => cpu.set_reg(rd, ((s1 as i64) * (s2 as i64) >> 32) as i32),
            RType::Mulhu => cpu.set_reg(rd, ((s1 as u64) * (s2 as u64) >> 32) as i32),
            RType::Mulhsu => cpu.set_reg(rd, ((s1 as i64) * ((s2 as u64) as i64) >> 32) as i32),
            RType::Div => cpu.set_reg(rd, s1 / s2),
            RType::Rem => cpu.set_reg(rd, s1 % s2),
            // bitwise logic
            RType::And => cpu.set_reg(rd, s1 & s2),
            RType::Or => cpu.set_reg(rd, s1 | s2),
            RType::Xor => cpu.set_reg(rd, s1 ^ s2),
            // shift
            RType::Sll => cpu.set_reg(rd, s1 << s2),
            RType::Srl => cpu.set_reg(rd, ((s1 as u32) >> (s2 as u32)) as i32),
            RType::Sra => cpu.set_reg(rd, s1 >> s2),
        }
        cpu.pc += 4;
    }

    pub fn opcode(&self) -> (u32, u32, u32) {
        match self {
            // arithmetic
            RType::Add => (0b0110011, 0x0, 0x00),
            RType::Sub => (0b0110011, 0x0, 0x20),
            RType::Mul => (0b0110011, 0x0, 0x00),
            RType::Mulh => (0b0110011, 0x1, 0x01),
            RType::Mulhu => (0b0110011, 0x3, 0x01),
            RType::Mulhsu => (0b0110011, 0x2, 0x01),
            RType::Div => (0b0110011, 0x4, 0x01),
            RType::Rem => (0b0110011, 0x6, 0x01),
            // bitwise logic
            RType::And => (0b0110011, 0x7, 0x00),
            RType::Or => (0b0110011, 0x6, 0x00),
            RType::Xor => (0b0110011, 0x4, 0x00),
            // shift
            RType::Sll => (0b0110011, 0x1, 0x00),
            RType::Srl => (0b0110011, 0x5, 0x00),
            RType::Sra => (0b0110011, 0x5, 0x20),
        }
    }
}
