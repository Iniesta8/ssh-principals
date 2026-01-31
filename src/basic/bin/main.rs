use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return;
    }

    let user = &args[1];
    let host = &args[2];

    println!("{}@{}", user, host);
}
