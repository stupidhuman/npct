use std::env;

fn main() {
    // 这条 注释来自于 test分支
    let args = env::args();

    println!("args: {:?}", args);
}
