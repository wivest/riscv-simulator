use super::common::*;
use super::token::{label_ref, offset, register};

use crate::language::{
    instruction::*,
    token::{Immediate, Offset},
};

pub fn mv<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("mv")
        .name_then(register())
        .then_ignore(just(","))
        .then(register())
        .map(|(rd, rs)| {
            vec![Instruction::IType {
                name: IType::Addi,
                rd,
                rs,
                imm: Immediate::Value(0),
            }]
        })
}

// arithmetic
pub fn neg<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("neg")
        .name_then(register())
        .then_ignore(just(","))
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
pub fn not<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("not")
        .name_then(register())
        .then_ignore(just(","))
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
    let num = number(32, i32::from_le_bytes).map(|n| {
        (
            Immediate::Value((n + 0x800) >> 12),
            Immediate::Value(n << 20 >> 20),
        )
    });
    let equ = text::ident()
        .inline()
        .map(|s| (Immediate::EquUpper(s), Immediate::Equ12(s)));

    just("li")
        .name_then(register())
        .then_ignore(just(","))
        .then(choice((num, equ)))
        .map(move |(rd, (upp, low))| {
            vec![
                Instruction::UType {
                    name: UType::Lui,
                    rd,
                    imm: upp,
                },
                Instruction::IType {
                    name: IType::Addi,
                    rd,
                    rs: rd,
                    imm: low,
                },
            ]
        })
}

pub fn la<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("la")
        .name_then(register())
        .then_ignore(just(","))
        .then(label_ref())
        .map(move |(rd, label)| {
            vec![
                Instruction::UType {
                    name: UType::Lui,
                    rd,
                    imm: Immediate::UpperPseudo(label),
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
    just("j").name_then(offset(21)).map(|imm| {
        vec![Instruction::JType {
            name: JType::Jal,
            rd: 0,
            imm,
        }]
    })
}

pub fn call<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("call").name_then(label_ref()).map(|label| {
        vec![
            Instruction::UType {
                name: UType::Auipc,
                rd: 1,
                imm: Immediate::PcrelHi(label),
            },
            Instruction::IType {
                name: IType::Jalr,
                rd: 1,
                rs: 1,
                imm: Immediate::PcrelLo(label),
            },
        ]
    })
}

pub fn tail<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("tail").name_then(label_ref()).map(|label| {
        vec![
            Instruction::UType {
                name: UType::Auipc,
                rd: 6,
                imm: Immediate::PcrelHi(label),
            },
            Instruction::IType {
                name: IType::Jalr,
                rd: 0,
                rs: 6,
                imm: Immediate::PcrelLo(label),
            },
        ]
    })
}

pub fn ret<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("ret").to(vec![Instruction::IType {
        name: IType::Jalr,
        rd: 0,
        rs: 1,
        imm: Immediate::Value(0),
    }])
}

pub fn jal<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("jal").name_then(offset(21)).map(|imm| {
        vec![Instruction::JType {
            name: JType::Jal,
            rd: 1,
            imm,
        }]
    })
}

// branch
pub fn beqz<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("beqz")
        .ignore_then(register())
        .then_ignore(just(","))
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

pub fn bnez<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("bnez")
        .ignore_then(register())
        .then_ignore(just(","))
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

pub fn bltz<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("bltz")
        .ignore_then(register())
        .then_ignore(just(","))
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

pub fn bgt<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("bgt")
        .ignore_then(register())
        .then_ignore(just(","))
        .then(register())
        .then_ignore(just(","))
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

pub fn bgtu<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("bgtu")
        .ignore_then(register())
        .then_ignore(just(","))
        .then(register())
        .then_ignore(just(","))
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

pub fn bgtz<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("bgtz")
        .ignore_then(register())
        .then_ignore(just(","))
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

pub fn ble<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("ble")
        .ignore_then(register())
        .then_ignore(just(","))
        .then(register())
        .then_ignore(just(","))
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

pub fn bleu<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("bleu")
        .ignore_then(register())
        .then_ignore(just(","))
        .then(register())
        .then_ignore(just(","))
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

pub fn blez<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("blez")
        .ignore_then(register())
        .then_ignore(just(","))
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

pub fn bgez<'src>() -> impl StrParser<'src, Vec<Instruction<Immediate<'src>, Offset<'src>>>> {
    just("beqz")
        .ignore_then(register())
        .then_ignore(just(","))
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
        mv(),
        neg(),
        not(),
        li(),
        la(),
        j(),
        call(),
        tail(),
        ret(),
        jal(),
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
    .boxed()
}
