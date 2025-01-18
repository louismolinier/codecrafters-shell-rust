use std::env;
use std::path::Path;

pub fn cd(args: Vec<&str>) {
    if args.len() != 2 {
        println!("cd: too many arguments");
    } else {
        let path = Path::new(args[1]);
        if path.exists() {
            env::set_current_dir(args[1]).expect("");
        } else {
            println!("cd: {}: No such file or directory", args[1]);
        }
    }
}
