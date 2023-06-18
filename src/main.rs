use std::io;
use std::io::Write;

mod file_system;
mod node;

use crate::file_system::KfileSystem;

fn main() {
    let mut fs = KfileSystem::new();

    let mut cmd = String::new();
    while cmd.trim() != "quit" {
        cmd.clear();
        print!("{}$ ", fs.pwd().unwrap());

        io::stdout().flush().expect("Cannot flush stdout");
        io::stdin()
            .read_line(&mut cmd)
            .expect("Cannot read user input");

        fs.call(&cmd);
    }
}
