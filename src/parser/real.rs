use super::common::*;
use super::token::{immediate12, immediate20, offset, register};

use crate::language::instruction::*;
use crate::language::token::{Immediate, Offset};

fn btype<'src>(
    name: BType,
    prefix: impl StrParser<'src, &'src str>,
) -> impl StrParser<'src, Instruction<Immediate<'src>, Offset<'src>>> {
    prefix
        .name_then(register().then_arg(register()).then_arg(offset(13)))
        .map(move |((rs1, rs2), offset)| Instruction::BType {
            name,
            rs1,
            rs2,
            offset,
        })
}

fn itype<'src>(
    name: IType,
    prefix: impl StrParser<'src, &'src str>,
) -> impl StrParser<'src, Instruction<Immediate<'src>, Offset<'src>>> {
    prefix
        .name_then(register().then_arg(register()).then_arg(immediate12()))
        .map(move |((rd, rs), imm)| Instruction::IType { name, rd, rs, imm })
}

fn itype_load<'src>(
    name: IType,
    prefix: impl StrParser<'src, &'src str>,
) -> impl StrParser<'src, Instruction<Immediate<'src>, Offset<'src>>> {
    let load = immediate12().index(register());
    prefix
        .name_then(register().then_arg(load))
        .map(move |(rd, (imm, rs))| Instruction::IType { name, rd, rs, imm })
}

fn jalr<'src>() -> impl StrParser<'src, Instruction<Immediate<'src>, Offset<'src>>> {
    let jump = immediate12().index(register());
    just("jalr")
        .name_then(register().then_arg(jump))
        .map(move |(rd, (imm, rs))| Instruction::IType {
            name: IType::Jalr,
            rd,
            rs,
            imm,
        })
}

fn jtype<'src>(
    name: JType,
    prefix: impl StrParser<'src, &'src str>,
) -> impl StrParser<'src, Instruction<Immediate<'src>, Offset<'src>>> {
    prefix
        .name_then(register().then_arg(offset(21)))
        .map(move |(rd, imm)| Instruction::JType { name, rd, imm })
}

fn rtype<'src>(
    name: RType,
    prefix: impl StrParser<'src, &'src str>,
) -> impl StrParser<'src, Instruction<Immediate<'src>, Offset<'src>>> {
    prefix
        .name_then(register().then_arg(register()).then_arg(register()))
        .map(move |((rd, rs1), rs2)| Instruction::RType { name, rd, rs1, rs2 })
}

fn stype<'src>(
    name: SType,
    prefix: impl StrParser<'src, &'src str>,
) -> impl StrParser<'src, Instruction<Immediate<'src>, Offset<'src>>> {
    let store = immediate12().index(register());
    prefix
        .name_then(register().then_arg(store))
        .map(move |(rs2, (imm, rs1))| Instruction::SType {
            name,
            rs1,
            rs2,
            imm,
        })
}

fn utype<'src>(
    name: UType,
    prefix: impl StrParser<'src, &'src str>,
) -> impl StrParser<'src, Instruction<Immediate<'src>, Offset<'src>>> {
    prefix
        .name_then(register().then_arg(immediate20()))
        .map(move |(rd, imm)| Instruction::UType { name, rd, imm })
}

fn system<'src>() -> impl StrParser<'src, Instruction<Immediate<'src>, Offset<'src>>> {
    let ebreak = just("ebreak").map(|_| Instruction::System(System::Ebreak));

    choice((ebreak,))
}

macro_rules! instructions {
    ($func:expr, $en:ident, [ $($var:ident),+ $(,)?]) => {
        choice(($({
            let name: &'static str = stringify!($var).to_lowercase().leak();
            $func($en::$var, just(name))
        },)+)).boxed()
    };
}

pub fn real_instructions<'src>() -> impl StrParser<'src, Instruction<Immediate<'src>, Offset<'src>>>
{
    let btype_ins = instructions!(btype, BType, [Beq, Bne, Blt, Bltu, Bge, Bgeu]);
    let iitype_ins = instructions!(itype, IType, [Addi, Andi, Ori, Xori, Slli, Srli, Srai]);
    let iltype_ins = instructions!(itype_load, IType, [Lw, Lh, Lhu, Lb, Lbu]);
    let itype_ins = choice((iitype_ins, iltype_ins, jalr()));
    let jtype_ins = instructions!(jtype, JType, [Jal]);
    let rtype_ins = instructions!(
        rtype,
        RType,
        [
            Add, Sub, Mul, Mulh, Mulhu, Mulhsu, Div, Rem, And, Or, Xor, Sll, Srl, Sra,
        ]
    );
    let stype_ins = instructions!(stype, SType, [Sw, Sh, Sb]);
    let utype_ins = instructions!(utype, UType, [Lui, Auipc]);
    let system_ins = system();

    choice((
        btype_ins, itype_ins, jtype_ins, rtype_ins, stype_ins, utype_ins, system_ins,
    ))
}
