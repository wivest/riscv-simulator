use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::error::Rich;
use std::{
    fs::OpenOptions,
    io::{Error, Read},
};

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

    match cli::load(&content) {
        Ok(mut proc) => cli::run_repl(&mut proc),
        Err(errors) => report_errors(errors, &path, &content),
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
