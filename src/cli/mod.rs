use crate::linker::Linker;
use crate::parser;
use crate::processor::Processor;
use chumsky::{Parser, error::Rich};
use command::Command;
use std::io::Write;

pub mod command;

const RESET: u32 = 0x200;

pub fn load<'a>(content: &'a String) -> Result<Processor, Vec<Rich<'a, char>>> {
    let result = parser::program((RESET, 0, 0, 0))
        .parse(&content)
        .into_result();

    let program = match result {
        Ok(program) => program,
        Err(errors) => return Err(errors),
    };

    let mut linker = Linker::new();
    for sect in vec![program.text, program.data, program.rodata, program.bss] {
        linker.import_section(sect);
    }
    Ok(Processor::new(RESET, linker.link()))
}

pub fn run_repl(proc: &mut Processor) {
    loop {
        match get_command() {
            Command::Help => println!("TODO help"),
            Command::Quit => return,
            Command::Exec(com) => proc.execute(com),
        }
    }
}

fn get_command() -> Command {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        if let Some(command) = Command::parse(buf) {
            return command;
        }
        println!("Invalid command!")
    }
}
