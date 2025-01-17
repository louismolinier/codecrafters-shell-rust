use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::ExitCode;

mod exit;
use exit::exit;

mod echo;
use echo::echo;

mod r#type;
use r#type::type_builtin;

mod exec_non_builtin;
use exec_non_builtin::exec_non_builtin;

fn main() -> ExitCode {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input.pop();

        let path_env = env::var("PATH").unwrap_or("".to_string());
        let paths: Vec<&str> = path_env.split(":").collect();

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
                "type" => type_builtin(args, paths),
                _ => exec_non_builtin(args, paths),
            }
        }
    }
}
