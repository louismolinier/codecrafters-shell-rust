use std::process::ExitCode;

pub fn exit(args: &Vec<String>) -> (bool, ExitCode) {
    match args.len() {
        1 => return (true, ExitCode::from(0)),
        2 => match args[1].parse::<u8>() {
            Ok(code) => return (true, ExitCode::from(code)),
            Err(_) => return (true, ExitCode::from(0)),
        },
        _ => {
            println!("exit: too many arguments");
            return (false, ExitCode::from(0));
        }
    }
}
