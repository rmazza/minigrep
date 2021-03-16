use std::env;
use std::error::Error;
use std::path::Path;
use std::process;

use minigrep::Command;
use minigrep::Search;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Search::new(&args);
    
    
    // .unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: dyn Command) -> Result<(), Box<dyn Error>> {

}
