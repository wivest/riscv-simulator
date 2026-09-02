use chumsky::prelude::*;

#[derive(Clone)]
pub enum Command {
    Step(u32),
    Run,
    Output,
    Memory,
    Registers,
    All,
    Quit,
    Help,
}

fn step<'src>() -> impl Parser<'src, &'src str, Command> {
    just("/step")
        .ignore_then(text::inline_whitespace().at_least(1))
        .ignore_then(text::int(10))
        .map(|s| u32::from_str_radix(s, 10).unwrap())
        .filter(|n| *n > 0)
        .map(|n| Command::Step(n))
}

impl Command {
    pub fn parse(input: String) -> Option<Command> {
        let parser = choice((
            step(),
            just("/run").to(Command::Run),
            just("/out").to(Command::Output),
            just("/mem").to(Command::Memory),
            just("/reg").to(Command::Registers),
            just("/all").to(Command::All),
            choice((just("/quit"), just("/exit"))).to(Command::Quit),
            just("/help").to(Command::Help),
        ))
        .padded();
        parser.parse(&input).into_output()
    }
}
