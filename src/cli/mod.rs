use command::Command;

pub mod command;

pub fn get_command() -> Command {
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        if let Some(command) = Command::parse(buf) {
            return command;
        }
        println!("Invalid command!")
    }
}
