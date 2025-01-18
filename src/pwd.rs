use std::env;

pub fn pwd(args: Vec<&str>) {
    if args.len() != 1 {
        println!("pwd: too many arguments")
    } else {
        let current_path = env::current_dir();
        println!("{}", current_path.expect("").display());
    }
}
