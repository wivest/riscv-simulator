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

    let to_parse = "add x1, x2, x3";
    let result = chumsky_parser::add().parse(to_parse);
    println!("{:?}", result.unwrap());
}
