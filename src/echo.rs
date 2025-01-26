pub fn echo(args: &Vec<String>) {
    for i in 1..args.len() - 1 {
        print!("{} ", args[i]);
    }
    println!("{}", args[args.len() - 1]);
}
