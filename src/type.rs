use crate::utils::is_in_folder;

pub fn type_builtin(args: &Vec<String>, paths: Vec<&str>) {
    for i in 1..args.len() {
        match args[i].as_str() {
            "echo" | "exit" | "type" | "pwd" => println!("{} is a shell builtin", args[i]),
            _ => {
                let mut found = false;
                for folder in &paths {
                    if is_in_folder(folder, &args[i]) {
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
