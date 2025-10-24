use chumsky::prelude::*;

use crate::processor::Processor;

#[derive(Debug, Clone, Copy)]
pub enum RType {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

#[derive(Debug, Clone, Copy)]
pub enum IType {
    Addi,
}

#[derive(Debug, Clone, Copy)]
pub enum BType {
    Beq,
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
    BType {
        name: BType,
        rs1: i32,
        rs2: i32,
        offset: i32,
    },
}

impl Instruction {
    pub fn execute(&self, cpu: &mut Processor) {
        match *self {
            Instruction::RType { name, rd, rs1, rs2 } => {
                match name {
                    RType::Add => {
                        cpu.registers[rd as usize] =
                            cpu.registers[rs1 as usize] + cpu.registers[rs2 as usize];
                    }
                    RType::Sub => {
                        cpu.registers[rd as usize] =
                            cpu.registers[rs1 as usize] - cpu.registers[rs2 as usize];
                    }
                    RType::Mul => {
                        cpu.registers[rd as usize] =
                            cpu.registers[rs1 as usize] * cpu.registers[rs2 as usize];
                    }
                    RType::Div => {
                        cpu.registers[rd as usize] =
                            cpu.registers[rs1 as usize] / cpu.registers[rs2 as usize];
                    }
                    RType::Rem => {
                        cpu.registers[rd as usize] =
                            cpu.registers[rs1 as usize] % cpu.registers[rs2 as usize];
                    }
                }
                cpu.pc += 4;
            }
            Instruction::IType { name, rd, rs, imm } => {
                match name {
                    IType::Addi => {
                        cpu.registers[rd as usize] = cpu.registers[rs as usize] + imm;
                    }
                }
                cpu.pc += 4;
            }
            Instruction::BType {
                name,
                rs1,
                rs2,
                offset,
            } => match name {
                BType::Beq => {
                    let left = cpu.registers[rs1 as usize];
                    let right = cpu.registers[rs2 as usize];
                    if left == right {
                        let pc = cpu.pc as i32;
                        cpu.pc = (pc + offset) as usize;
                    } else {
                        cpu.pc += 4;
                    }
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
    let sign = just("-").map(|_| -1).or(empty().map(|_| 1));
    let number = text::int(10).map(|s: &'src str| s.parse::<i32>().unwrap());
    sign.then_ignore(text::whitespace())
        .then(number)
        .map(|(s, n)| s * n)
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
    let mul = rtype(RType::Mul, just("mul"));
    let div = rtype(RType::Div, just("div"));
    let rem = rtype(RType::Rem, just("rem"));
    let addi = itype(IType::Addi, just("addi"));
    let instruction = choice((add, sub, mul, div, rem, addi));

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

    #[test]
    fn test_immediate() {
        let result = immediate().parse("42");
        assert_eq!(result.unwrap(), 42);
        let result = immediate().parse("-42");
        assert_eq!(result.unwrap(), -42);
        let result = immediate().parse("- 42");
        assert_eq!(result.unwrap(), -42);
    }
}
