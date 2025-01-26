use std::env;
use std::path::Path;

pub fn cd(args: &Vec<String>) {
    if args.len() != 2 {
        println!("cd: too many arguments");
    } else {
        let mut path_str = args[1].clone();
        if path_str.starts_with('~') {
            path_str.remove(0);
            let home_env = env::var("HOME").unwrap_or("".to_string());
            path_str = format!("{}{}", home_env, path_str);
        }
        let path = Path::new(&path_str);
        if path.exists() {
            env::set_current_dir(path_str).expect("");
        } else {
            println!("cd: {}: No such file or directory", args[1]);
        }
    }
}
