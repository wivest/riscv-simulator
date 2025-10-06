use crate::grammar::Rule;
use pest::iterators::Pairs;

pub struct Language {
    instructions: Vec<Instruction>,
}

impl Language {
    pub fn new(pairs: Pairs<Rule>) -> Self {
        let instructions = parse_program(pairs);
        Language {
            instructions: vec![],
        }
    }
}

fn parse_program(pairs: Pairs<Rule>) -> Option<Vec<Instruction>> {
    let program = pairs.peek()?;
    match program.as_rule() {
        Rule::program => parse_instructions(program.into_inner()),
        _ => None,
    }
}

fn parse_instructions(pairs: Pairs<Rule>) -> Option<Vec<Instruction>> {
    let mut list = vec![];

    for pair in pairs {
        match pair.as_rule() {
            Rule::instruction => {
                let instruction = pair.into_inner();
                for rule in instruction {
                    let i = match rule.as_rule() {
                        Rule::add => {
                            println!("add!");
                            Some(Instruction::Add)
                        }
                        _ => None,
                    };
                    list.push(i?)
                }
            }
            _ => {
                println!("other ({:?})...", pair.as_rule());
            }
        }
    }

    Some(list)
}

enum Instruction {
    Add,
}
