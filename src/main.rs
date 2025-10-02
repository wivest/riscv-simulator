use grammar::{Grammar, Rule};
use pest::Parser;
use std::fs::OpenOptions;
use std::io::{Error, Read};

mod grammar;

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

    match open_file(path) {
        Ok(content) => {
            println!("File content:\n{}", content);
            let pairs = Grammar::parse(Rule::add, &content);
            match pairs {
                Ok(pairs) => {
                    for pair in pairs {
                        println!("{}", pair);
                    }
                }
                Err(e) => println!("Error!\n{}", e),
            }
        }
        Err(e) => println!("Error {}!", e),
    }
}
