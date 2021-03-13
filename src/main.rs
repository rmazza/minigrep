use std::env;
use std::error::Error;
use std::path::Path;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&config.directory);

    Config::search(&path, &config.query, config.case_insensitive, &print)?;
    Ok(())
}

fn print(path: &str) {
    println!("{}", path);
}
