use chumsky::Parser;
use processor::Processor;
use std::fs::OpenOptions;
use std::io::{Error, Read};

mod directive;
mod instruction;
mod linker;
mod names;
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
            Ok((strings, bytes, instrs, defs)) => {
                let mut proc = Processor::new(1024);
                proc.store_strings(strings);
                proc.store_bytes(bytes);
                proc.store_instrs(linker::translate(instrs, defs));
                proc.execute();
                println!("{:?}", proc.memory);
            }
            Err(err) => println!("{err:?}"),
        }
    } else {
        println!("File error!");
    }
}
