use super::common::*;
use super::token::{label_ref, offset, register};

use crate::language::{
    instruction::*,
    token::{Immediate, Offset},
};

// arithmetic
pub fn neg<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>>
{
    just("neg")
        .ignore_then(register())
        .then(register())
        .map(|(rd, rs2)| {
            vec![Instruction::RType {
                name: RType::Sub,
                rd,
                rs1: 0,
                rs2,
            }]
        })
}

// bitwise logic
pub fn not<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>>
{
    just("not")
        .ignore_then(register())
        .then(register())
        .map(|(rd, rs)| {
            vec![Instruction::IType {
                name: IType::Xori,
                rd,
                rs,
                imm: Immediate::Value(-1),
            }]
        })
}

// load
pub fn li<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("li")
        .ignore_then(register())
        .then_ignore(just(","))
        .then(number(32, i32::from_le_bytes))
        .map(move |(rd, imm)| {
            vec![
                Instruction::UType {
                    name: UType::Lui,
                    rd,
                    imm: Immediate::Value(imm >> 12),
                },
                Instruction::IType {
                    name: IType::Addi,
                    rd,
                    rs: 0,
                    imm: Immediate::Value(imm << 20 >> 20), // TODO: test for negative edge case
                },
            ]
        })
}

pub fn la<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("la")
        .ignore_then(register())
        .then_ignore(just(","))
        .then(label_ref())
        .map(move |(rd, label)| {
            vec![
                Instruction::UType {
                    name: UType::Lui,
                    rd,
                    imm: Immediate::Upper(label),
                },
                Instruction::IType {
                    name: IType::Addi,
                    rd,
                    rs: rd,
                    imm: Immediate::Lower(label),
                },
            ]
        })
}

// jump
pub fn j<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("j").ignore_then(offset(20)).map(|imm| {
        vec![Instruction::JType {
            name: JType::Jal,
            rd: 0,
            imm,
        }]
    })
}

pub fn call<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>>
{
    just("call").ignore_then(label_ref()).map(|label| {
        vec![
            Instruction::UType {
                name: UType::Auipc,
                rd: 1,
                imm: Immediate::Upper(label),
            },
            Instruction::IType {
                name: IType::Jalr,
                rd: 1,
                rs: 1,
                imm: Immediate::Lower(label),
            },
        ]
    })
}

pub fn tail<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>>
{
    just("call").ignore_then(label_ref()).map(|label| {
        vec![
            Instruction::UType {
                name: UType::Auipc,
                rd: 6,
                imm: Immediate::Upper(label),
            },
            Instruction::IType {
                name: IType::Jalr,
                rd: 0,
                rs: 6,
                imm: Immediate::Lower(label),
            },
        ]
    })
}

pub fn ret<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>>
{
    just("ret").to(vec![Instruction::IType {
        name: IType::Jalr,
        rd: 0,
        rs: 1,
        imm: Immediate::Value(0),
    }])
}

// branch
pub fn beqz<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>>
{
    just("beqz")
        .ignore_then(register())
        .then(offset(12))
        .map(|(rs1, offset)| {
            vec![Instruction::BType {
                name: BType::Beq,
                rs1,
                rs2: 0,
                offset,
            }]
        })
}

pub fn bnez<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>>
{
    just("bnez")
        .ignore_then(register())
        .then(offset(12))
        .map(|(rs1, offset)| {
            vec![Instruction::BType {
                name: BType::Bne,
                rs1,
                rs2: 0,
                offset,
            }]
        })
}

pub fn bltz<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>>
{
    just("bltz")
        .ignore_then(register())
        .then(offset(12))
        .map(|(rs1, offset)| {
            vec![Instruction::BType {
                name: BType::Blt,
                rs1,
                rs2: 0,
                offset,
            }]
        })
}

pub fn bgt<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>>
{
    just("bgt")
        .ignore_then(register())
        .then(register())
        .then(offset(12))
        .map(|((rs1, rs2), offset)| {
            vec![Instruction::BType {
                name: BType::Blt,
                rs1: rs2,
                rs2: rs1,
                offset,
            }]
        })
}

pub fn bgtu<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>>
{
    just("bgtu")
        .ignore_then(register())
        .then(register())
        .then(offset(12))
        .map(|((rs1, rs2), offset)| {
            vec![Instruction::BType {
                name: BType::Bltu,
                rs1: rs2,
                rs2: rs1,
                offset,
            }]
        })
}

pub fn bgtz<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>>
{
    just("bgtz")
        .ignore_then(register())
        .then(offset(12))
        .map(|(rs2, offset)| {
            vec![Instruction::BType {
                name: BType::Blt,
                rs1: 0,
                rs2,
                offset,
            }]
        })
}

pub fn ble<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>>
{
    just("ble")
        .ignore_then(register())
        .then(register())
        .then(offset(12))
        .map(|((rs1, rs2), offset)| {
            vec![Instruction::BType {
                name: BType::Bge,
                rs1: rs2,
                rs2: rs1,
                offset,
            }]
        })
}

pub fn bleu<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>>
{
    just("bleu")
        .ignore_then(register())
        .then(register())
        .then(offset(12))
        .map(|((rs1, rs2), offset)| {
            vec![Instruction::BType {
                name: BType::Bgeu,
                rs1: rs2,
                rs2: rs1,
                offset,
            }]
        })
}

pub fn blez<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>>
{
    just("blez")
        .ignore_then(register())
        .then(offset(12))
        .map(|(rs2, offset)| {
            vec![Instruction::BType {
                name: BType::Bge,
                rs1: 0,
                rs2,
                offset,
            }]
        })
}

pub fn bgez<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>>
{
    just("beqz")
        .ignore_then(register())
        .then(offset(12))
        .map(|(rs1, offset)| {
            vec![Instruction::BType {
                name: BType::Beq,
                rs1,
                rs2: 0,
                offset,
            }]
        })
}

pub fn pseudo_instructions<'src>()
-> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    choice((
        neg(),
        not(),
        li(),
        la(),
        j(),
        call(),
        tail(),
        ret(),
        beqz(),
        bnez(),
        bltz(),
        bgt(),
        bgtu(),
        bgtz(),
        ble(),
        bleu(),
        blez(),
        bgez(),
    ))
}
