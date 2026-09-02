use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{Parser, error::Rich};
use processor::Processor;
use std::fs::OpenOptions;
use std::io::{Error, Read};

use crate::cli::command::Command;
use crate::linker::Linker;

mod language {
    pub mod directive;
    pub mod instruction;
    pub mod names;
    pub mod token;
    pub mod word;
}
mod cli;
mod linker;
mod parser;
mod processor;

const RESET: u32 = 0x200;

fn open_file(path: &str) -> Result<String, Error> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or("examples/source.asm".to_owned());

    let Ok(content) = open_file(&path) else {
        println!("File error!");
        return;
    };

    let result = parser::program((RESET, 0, 0, 0))
        .parse(&content)
        .into_result();

    let program = match result {
        Ok(program) => program,
        Err(errors) => {
            report_errors(errors, &path, &content);
            return;
        }
    };

    let mut linker = Linker::new();
    for sect in vec![program.text, program.data, program.rodata, program.bss] {
        linker.import_section(sect);
    }
    let mut proc = Processor::new(RESET, linker.link());

    loop {
        let com = cli::get_command();
        match com {
            Command::Help => println!("TODO help"),
            Command::Quit => return,
            Command::Exec(com) => proc.execute(com),
        }
    }
}

fn report_errors(errors: Vec<Rich<'_, char>>, path: &String, content: &String) {
    for err in errors {
        Report::build(ReportKind::Error, (path, err.span().into_range()))
            .with_message(err.to_string())
            .with_label(
                Label::new((path, err.span().into_range()))
                    .with_message(format!("{}", err.reason()))
                    .with_color(Color::Red),
            )
            .finish()
            .print((path, Source::from(content)))
            .unwrap();
    }
}
