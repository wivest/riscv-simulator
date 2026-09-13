use chumsky::prelude::*;
use rvsim::{
    linker::Linker,
    parser,
    processor::{Processor, RESET, command::Executable},
};
use std::{fs::OpenOptions, io::Read};

fn open_file(path: &str) -> String {
    let mut file = OpenOptions::new().read(true).open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

pub fn load<'a>(content: &'a String) -> Result<Processor, Vec<Rich<'a, char>>> {
    let program = parser::program((RESET, 0, 0, 0))
        .parse(&content)
        .into_result()?;

    let mut linker = Linker::new();
    for sect in vec![program.text, program.data, program.rodata, program.bss] {
        linker.import_section(sect)?;
    }
    Ok(Processor::new(RESET, linker.link()?))
}

#[test]
fn simple() {
    let content = open_file("tests/asm/simple.S");
    let mut cpu = load(&content).unwrap();
    cpu.execute(Executable::Run);
    assert_eq!(cpu.get_reg(1), 6);
    assert_eq!(cpu.get_reg(2), 4);
    assert_eq!(cpu.get_reg(3), 10);
    assert_eq!(cpu.get_reg(4), 2);
    assert_eq!(cpu.get_reg(5), 4);
    assert_eq!(cpu.get_reg(6), 8);
}

#[test]
fn replace() {
    let content = open_file("tests/asm/replace.S");
    let mut cpu = load(&content).unwrap();
    cpu.execute(Executable::Run);

    let org = 0x1000;
    let target = "hello,from,assembly!";
    for (i, byte) in target.bytes().enumerate() {
        assert_eq!(cpu.get_memory(org + i as u32), Some(byte));
    }
}

#[test]
fn printer() {
    let content = open_file("tests/asm/printer.S");
    let mut cpu = load(&content).unwrap();
    cpu.execute(Executable::Run);

    let target = "this is from printer.S file";
    assert_eq!(cpu.stdout(), target.to_owned());
}

#[test]
fn empty() {
    let content = open_file("tests/asm/empty.S");
    let cpu = load(&content).unwrap();
    assert_eq!(cpu.get_memory(0x200), None);
}
