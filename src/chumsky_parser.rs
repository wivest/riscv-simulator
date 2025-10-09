use chumsky::prelude::*;

#[derive(Debug)]
pub enum Instruction {
    Add { rd: i32, rs1: i32, rs2: i32 },
}

pub fn add<'src>() -> impl Parser<'src, &'src str, Instruction> {
    let register = just("x")
        .ignore_then(text::int(10))
        .padded()
        .map(|s: &'src str| s.parse::<i32>().unwrap());

    let add_parser = just("add")
        .ignore_then(register.separated_by(just(",")).collect_exactly::<[_; 3]>())
        .map(|[rd, rs1, rs2]| Instruction::Add {
            rd: rd,
            rs1: rs1,
            rs2: rs2,
        });

    add_parser
}
