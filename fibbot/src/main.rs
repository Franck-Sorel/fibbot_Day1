use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input1> <input2>", args[0]);
        std::process::exit(1);
    }

    let input1 = &args[1];
    let input2 = &args[2];

    println!("Input 1: {}", input1);
    println!("Input 2: {}", input2);
}
