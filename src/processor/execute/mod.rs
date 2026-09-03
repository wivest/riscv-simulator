use super::Processor;
use crate::language::instruction::Instruction;
use crate::language::names::*;

mod btype;
mod itype;
mod jtype;
mod rtype;
mod stype;
mod utype;

impl Instruction<i32, i32> {
    pub fn execute(&self, cpu: &mut Processor) {
        match *self {
            Instruction::BType {
                name,
                rs1,
                rs2,
                offset,
            } => name.execute(cpu, rs1, rs2, offset),
            Instruction::IType { name, rd, rs, imm } => name.execute(cpu, rd, rs, imm),
            Instruction::JType { name, rd, imm } => name.execute(cpu, rd, imm),
            Instruction::RType { name, rd, rs1, rs2 } => name.execute(cpu, rd, rs1, rs2),
            Instruction::SType {
                name,
                rs1,
                rs2,
                imm,
            } => name.execute(cpu, rs1, rs2, imm),
            Instruction::UType { name, rd, imm } => name.execute(cpu, rd, imm),
            Instruction::Ebreak => cpu.pc += 4,
        }
    }

    pub fn encode(&self) -> u32 {
        match *self {
            Instruction::BType {
                name,
                rs1,
                rs2,
                offset,
            } => {
                let (opcode, funct3) = name.opcode();
                let top = ((offset >> 6) as u32 & 0b100_0000) + ((offset >> 5) as u32 & 0b11_1111);
                let btm = (offset as u32 & 0b1_1110) + ((offset >> 11) as u32 & 0b1);
                (top << 25) + (rs2 << 20) + (rs1 << 15) + (funct3 << 12) + (btm << 7) + opcode
            }
            Instruction::IType { name, rd, rs, imm } => {
                let (opcode, funct3) = name.opcode();
                ((imm as u32) << 20) + (rs << 15) + (funct3 << 12) + (rd << 7) + opcode
            }
            Instruction::JType { name, rd, imm } => {
                let opcode = name.opcode();
                let bit20 = (imm >> 20) as u32 & 0b1;
                let top = (imm >> 1) as u32 & 0b11_1111_1111;
                let bit11 = (imm >> 11) as u32 & 0b1;
                let btm = (imm >> 12) as u32 & 0b1111_1111;
                (bit20 << 31) + (top << 21) + (bit11 << 20) + (btm << 12) + (rd << 7) + opcode
            }
            Instruction::RType { name, rd, rs1, rs2 } => {
                let (opcode, funct3, funct7) = name.opcode();
                (funct7 << 25) + (rs2 << 20) + (rs1 << 15) + (funct3 << 12) + (rd << 7) + opcode
            }
            Instruction::SType {
                name,
                rs1,
                rs2,
                imm,
            } => {
                let (opcode, funct3) = name.opcode();
                let top = (imm >> 5) as u32 & 0b111_1111;
                let btm = imm as u32 & 0b11111;
                (top << 25) + (rs2 << 20) + (rs1 << 15) + (funct3 << 12) + (btm << 7) + opcode
            }
            Instruction::UType { name, rd, imm } => {
                ((imm as u32) << 12) + (rd << 7) + name.opcode()
            }
            Instruction::Ebreak => u32::MAX,
        }
    }
}
