fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Invalid arguments. Usage: vincrementer version_string");
    }
    let input_version: String = args[1].clone();
    println!("{input_version}");
}
