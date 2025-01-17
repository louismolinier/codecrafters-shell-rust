use std::path::Path;
use std::process::Command;

fn is_in_folder(folder: &str, arg: &str) -> bool {
    let path = Path::new(folder).join(arg);
    return path.exists() && path.is_file();
}

pub fn exec_non_builtin(args: Vec<&str>, paths: Vec<&str>) {
    let command_args = args[1..].to_vec();
    let mut found = false;
    for folder in &paths {
        if is_in_folder(folder, args[0]) {
            let _ = Command::new(args[0]).args(command_args).spawn();
            found = true;
            break;
        }
    }
    if !found {
        println!("{}: command not found", args[0]);
    }
}
