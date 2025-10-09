use chumsky::prelude::*;

enum Instruction {
    Add { rd: i32, rs1: i32, rs2: i32 },
}

fn add<'src>() -> impl Parser<'src, &'src str, Instruction> {
    let register = just("x")
        .ignore_then(text::int(10))
        .map(|s: &'src str| s.parse::<i32>().unwrap());

    let add_parser = just("add")
        .ignore_then(register.repeated().collect_exactly::<[_; 3]>())
        .map(|[rd, rs1, rs2]| Instruction::Add {
            rd: rd,
            rs1: rs1,
            rs2: rs2,
        });

    add_parser
}
