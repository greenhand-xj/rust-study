use std::{env, process};
use mingrep::{Config};

fn main() {
    //解析命令行参数
    let args = env::args().collect::<Vec<String>>();
    // dbg!(args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}", err);
        process::exit(1);
    });
    
    if let Err(e) = mingrep::run(config) {
        eprintln!("Application error {}", e);
        process::exit(1);
    }
}