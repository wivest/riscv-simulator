use chumsky::prelude::*;

#[derive(Debug, Clone, Copy)]
enum RType {
    Add,
    Sub,
}

#[derive(Debug, Clone, Copy)]
enum IType {
    Addi,
}

#[derive(Debug)]
pub enum Instruction {
    RType {
        name: RType,
        rd: i32,
        rs1: i32,
        rs2: i32,
    },
    IType {
        name: IType,
        rd: i32,
        rs: i32,
        imm: i32,
    },
}

fn register<'src>() -> impl Parser<'src, &'src str, i32> {
    just("x")
        .ignore_then(text::int(10))
        .map(|s: &'src str| s.parse::<i32>().unwrap())
}

fn immediate<'src>() -> impl Parser<'src, &'src str, i32> {
    text::int(10).map(|s: &'src str| s.parse::<i32>().unwrap())
}

fn rtype<'src>(
    name: RType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction> {
    prefix
        .ignore_then(
            register()
                .padded()
                .separated_by(just(","))
                .collect_exactly::<[_; 3]>(),
        )
        .map(move |[rd, rs1, rs2]| Instruction::RType { name, rd, rs1, rs2 })
}

fn add<'src>() -> impl Parser<'src, &'src str, Instruction> {
    rtype(RType::Add, just("add"))
}

fn sub<'src>() -> impl Parser<'src, &'src str, Instruction> {
    rtype(RType::Sub, just("sub"))
}

pub fn program<'src>() -> impl Parser<'src, &'src str, Vec<Instruction>> {
    let instruction = choice((add(), sub()));

    instruction.padded().repeated().collect()
}
