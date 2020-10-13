use std::io::{stdin, stdout, Write, Result, Error};
use std::process::{Command, Child};

fn read() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    String::from(buf.trim())
}

fn tokenize() {}

fn parse() {}

fn expand() {}

fn redirect() {}

fn execute(program: String) -> Result<Child> {
    Command::new(program).spawn()
}

fn main() {
    loop {
        print!(">");
        stdout().flush();
        let program = read();
        match execute(program) {
            Ok(running) => { running.wait_with_output(); }
            Err(error) => { eprintln!("System error: {}", error) }
        }
    }
}
