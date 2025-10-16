use chumsky::Parser;
use std::fs::OpenOptions;
use std::io::{Error, Read};

mod chumsky_parser;
mod processor;

fn open_file(path: &str) -> Result<String, Error> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut content = String::from("");
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    let path = "examples/source.asm";

    if let Ok(content) = open_file(path) {
        let result = chumsky_parser::program().parse(&content).into_result();
        match result {
            Ok(instructions) => processor::execute(instructions),
            Err(err) => println!("{err:?}"),
        }
    } else {
        println!("File error!");
    }
}
