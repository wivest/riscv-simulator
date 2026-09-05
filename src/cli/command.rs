use crate::parser::{StrParser, common::number};
use chumsky::prelude::*;

#[derive(Clone)]
pub enum Command {
    Exec(Executable),
    Quit,
    Help,
}

#[derive(Clone)]
pub enum Executable {
    Goto(u32),
    Show(u32),
    Step(u32),
    Run,
    Output,
    Memory,
    Registers,
    Instructions,
}

fn with_arg<'src>(name: &'src str) -> impl StrParser<'src, u32> {
    just(name)
        .ignore_then(text::inline_whitespace().at_least(1))
        .ignore_then(number(32, u32::from_le_bytes))
        .filter(|n| *n > 0)
}

impl Command {
    pub fn parse(input: String) -> Option<Command> {
        let parser = choice((
            with_arg("goto").map(|n| Command::Exec(Executable::Goto(n))),
            with_arg("mem").map(|n| Command::Exec(Executable::Show(n))),
            with_arg("step").map(|n| Command::Exec(Executable::Step(n))),
            just("run").to(Command::Exec(Executable::Run)),
            just("out").to(Command::Exec(Executable::Output)),
            just("hex").to(Command::Exec(Executable::Memory)),
            just("reg").to(Command::Exec(Executable::Registers)),
            just("obj").to(Command::Exec(Executable::Instructions)),
            choice((just("quit"), just("exit"))).to(Command::Quit),
            just("help").to(Command::Help),
        ))
        .padded();
        parser.parse(&input).into_output()
    }
}
