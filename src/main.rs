use std::env;
use std::process;

use grep_remake::Config;

/// Main fuction of the program
fn main() {
    // Grabs arguments from environment
    let args: Vec<String> = env::args().collect();

    // Creates Config struct for execution
    let config = Config::new(&args)
        .unwrap_or_else(|err| {
            // Error handling -- if no arguments tell user in console
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    // Basic outputs
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    
    // Runs the grep with error check
    if let Err(e) = grep_remake::run(config) {
        // Error handling -- if anything goes wrong
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}