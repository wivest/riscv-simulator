use std::fs::OpenOptions;
use std::io::{Error, Read};

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
    let path = ".gitignore";

    match open_file(path) {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Error {}!", e),
    }
}
