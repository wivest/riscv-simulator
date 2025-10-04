use grammar::{Grammar, Rule};
use language::Language;
use pest::Parser;
use std::fs::OpenOptions;
use std::io::{Error, Read};

mod grammar;
mod language;

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
            let pairs = Grammar::parse(Rule::program, &content).unwrap();
            for pair in pairs {
                for p in pair.into_inner() {
                    println!("{}", p.as_str());
                }
            }
            let pairs = Grammar::parse(Rule::program, &content);
            match pairs {
                Ok(pairs) => {
                    let language = Language::new(pairs);
                }
                Err(e) => println!("Error!\n{}", e),
            }
        }
        Err(e) => println!("Error {}!", e),
    }
}
