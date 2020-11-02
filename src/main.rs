use std::env;
use npct::config::core::Config;

fn main() {
    let args = env::args();

    println!("args: {:?}", args);

    let config = Config::from_args(args);
}
