use std::env;
// use npct::config::core;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("args: {:?}", args);
}
