use chumsky::Parser;
use processor::Processor;
use std::fs::OpenOptions;
use std::io::{Error, Read};

use crate::linker::Linker;

mod language {
    pub mod directive;
    pub mod instruction;
    pub mod names;
    pub mod token;
    pub mod word;
}
mod linker;
mod parser;
mod processor;

fn open_file(path: &str) -> Result<String, Error> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut content = String::from("");
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or("examples/source.asm".to_owned());

    if let Ok(content) = open_file(&path) {
        let result = parser::program().parse(&content).into_result();
        match result {
            Ok(program) => {
                let mut proc = Processor::new(1024);
                let mut linker = Linker::new();
                for sect in vec![program.text, program.data, program.rodata, program.bss] {
                    linker.import_section(sect);
                }
                proc.memory = linker.link(&program.defs);
                proc.execute();
                println!("{}", proc.memory);
            }
            Err(err) => println!("{err:?}"),
        }
    } else {
        println!("File error!");
    }
}
