use std::env::current_dir;
use std::io::{stderr, stdin, stdout, Result, Write};
use std::process::{Child, Command};

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

fn cwd() -> String {
    if let Ok(os_cwd) = current_dir() {
        if let Some(os_cwd) = os_cwd.to_str() {
            return String::from(os_cwd);
        }
    }
    String::from("mush err 0")
}

fn main() {
    loop {
        print!("{}>", cwd());
        let _ = stdout().flush();
        let program = read();
        match execute(program) {
            Ok(running) => {
                let _ = running.wait_with_output();
            }
            Err(error) => {
                eprintln!("System error: {}", error);
                let _ = stderr().flush();
            }
        }
    }
}
