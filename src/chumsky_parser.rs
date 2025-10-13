use chumsky::prelude::*;

#[derive(Debug)]
pub enum Instruction {
    Add(InstructionType),
    Sub(InstructionType),
}
#[derive(Debug)]
pub enum InstructionType {
    RType { rd: i32, rs1: i32, rs2: i32 },
    IType { rd: i32, rs: i32, imm: i32 },
}

fn register<'src>() -> impl Parser<'src, &'src str, i32> {
    just("x")
        .ignore_then(text::int(10))
        .map(|s: &'src str| s.parse::<i32>().unwrap())
}

fn rtype<'src>(
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, InstructionType> {
    prefix
        .ignore_then(
            register()
                .padded()
                .separated_by(just(","))
                .collect_exactly::<[_; 3]>(),
        )
        .map(|[rd, rs1, rs2]| InstructionType::RType {
            rd: rd,
            rs1: rs1,
            rs2: rs2,
        })
}

fn add<'src>() -> impl Parser<'src, &'src str, Instruction> {
    rtype(just("add")).map(|rt| Instruction::Add(rt))
}

fn sub<'src>() -> impl Parser<'src, &'src str, Instruction> {
    rtype(just("sub")).map(|rt| Instruction::Sub(rt))
}

pub fn program<'src>() -> impl Parser<'src, &'src str, Vec<Instruction>> {
    let instruction = choice((add(), sub()));

    instruction.padded().repeated().collect()
}
