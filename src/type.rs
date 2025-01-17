pub fn type_builtin(args: Vec<&str>) {
    for i in 1..args.len() {
        match args[i] {
            "echo" | "exit" | "type" => println!("{} is a shell builtin", args[i]),
            _ => println!("{}: not found", args[i]),
        }
    }
}
