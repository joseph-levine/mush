use std::io::stdin;
use std::process::Command;

fn main() {
    print!(">");
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let program = buf.trim()
    Command::new(program).spawn().unwrap();
}
