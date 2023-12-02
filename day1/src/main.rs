use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a filename");
        return;
    }
    let filename = &args[1];

    println!("In file: {}", filename);
}
