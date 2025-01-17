#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::ExitCode;

mod exit;
use exit::exit;

mod echo;
use echo::echo;

fn main() -> ExitCode {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input.pop();

        let args: Vec<&str> = input.split_whitespace().collect();
        if args.len() != 0 {
            match args[0] {
                "exit" => {
                    let exit_result = exit(args);
                    if exit_result.0 {
                        return exit_result.1;
                    }
                }
                "echo" => echo(args),
                _ => println!("{}: command not found", args[0]),
            }
        }
    }
}
