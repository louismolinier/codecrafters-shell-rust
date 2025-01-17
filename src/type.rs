use std::path::Path;

fn is_in_folder(folder: &str, arg: &str) -> bool {
    let path = Path::new(folder).join(arg);
    return path.exists() && path.is_file();
}

pub fn type_builtin(args: Vec<&str>, paths: Vec<&str>) {
    for i in 1..args.len() {
        match args[i] {
            "echo" | "exit" | "type" => println!("{} is a shell builtin", args[i]),
            _ => {
                let mut found = false;
                for folder in &paths {
                    if is_in_folder(folder, args[i]) {
                        println!("{} is {}/{}", args[i], folder, args[i]);
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("{}: not found", args[i]);
                }
            }
        }
    }
}
