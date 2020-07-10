use std::process;
use std::env;

use timer::Config;

fn main() {
    let configuration = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });
    timer::run(configuration);
}
