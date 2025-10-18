use chumsky::prelude::*;

use crate::processor::Processor;

#[derive(Debug, Clone, Copy)]
pub enum RType {
    Add,
    Sub,
}

#[derive(Debug, Clone, Copy)]
pub enum IType {
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

impl Instruction {
    pub fn execute(self, cpu: &mut Processor) {
        match self {
            Instruction::RType { name, rd, rs1, rs2 } => match name {
                RType::Add => {
                    cpu.registers[rd as usize] =
                        cpu.registers[rs1 as usize] + cpu.registers[rs2 as usize];
                }
                RType::Sub => {
                    cpu.registers[rd as usize] =
                        cpu.registers[rs1 as usize] - cpu.registers[rs2 as usize];
                }
            },
            Instruction::IType { name, rd, rs, imm } => match name {
                IType::Addi => {
                    cpu.registers[rd as usize] = cpu.registers[rs as usize] + imm;
                }
            },
        }
    }
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

fn itype<'src>(
    name: IType,
    prefix: impl Parser<'src, &'src str, &'src str>,
) -> impl Parser<'src, &'src str, Instruction> {
    prefix
        .ignore_then(
            register()
                .padded()
                .separated_by(just(","))
                .collect_exactly::<[_; 2]>()
                .then_ignore(just(","))
                .then(immediate().padded()),
        )
        .map(move |([rd, rs], imm)| Instruction::IType { name, rd, rs, imm })
}

pub fn program<'src>() -> impl Parser<'src, &'src str, Vec<Instruction>> {
    let add = rtype(RType::Add, just("add"));
    let sub = rtype(RType::Sub, just("sub"));
    let addi = itype(IType::Addi, just("addi"));
    let instruction = choice((add, sub, addi));

    instruction.padded().repeated().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let result = register().parse("x10");
        assert_eq!(result.unwrap(), 10);
    }
}
