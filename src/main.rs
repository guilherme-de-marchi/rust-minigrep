use std::env;
use std::process;

fn main() {
    let args = env::args().collect();
    let conf = minigrep::Config::build(&args).unwrap_or_else(|err| {
        println!("Error building config: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(conf) {
        println!("Error running: {}", e);
        process::exit(1);
    }
}

