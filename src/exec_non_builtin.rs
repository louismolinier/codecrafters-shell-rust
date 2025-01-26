use crate::utils::is_in_folder;
use std::process::Command;

pub fn exec_non_builtin(args: &Vec<String>, paths: Vec<&str>) {
    let command = &args[0];
    let command_args = &args[1..];
    let mut found = false;
    for folder in &paths {
        if is_in_folder(folder, command) {
            let _ = Command::new(command)
                .args(command_args)
                .spawn()
                .expect("")
                .wait();
            found = true;
            break;
        }
    }
    if !found {
        println!("{}: command not found", args[0]);
    }
}
