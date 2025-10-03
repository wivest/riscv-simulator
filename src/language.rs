use crate::grammar::Rule;
use pest::iterators::Pairs;

pub struct Language {
    instructions: Vec<Instruction>,
}

impl Language {
    pub fn new(pairs: Pairs<Rule>) -> Self {
        for pair in pairs {
            match pair.as_rule() {
                Rule::program => {
                    let instructions = pair.into_inner();
                    for instruction in instructions {
                        match instruction.as_rule() {
                            Rule::instruction => {
                                let ins = instruction.into_inner();
                                for i in ins {
                                    match i.as_rule() {
                                        Rule::add => {
                                            println!("add!");
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {
                                println!("other...");
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Language {
            instructions: vec![],
        }
    }
}

enum Instruction {
    Add,
}
