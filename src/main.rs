use chumsky::Parser;
use std::fs::OpenOptions;
use std::io::{Error, Read};

mod chumsky_parser;

fn open_file(path: &str) -> Result<String, Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;
    let mut content = String::from("");
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    let path = "examples/source.asm";

    if let Ok(content) = open_file(path) {
        let result = chumsky_parser::program().parse(&content);
        println!("{:?}", result.unwrap());
    } else {
        println!("File error!");
    }
}
