use std::env;
use std::process;
use fogrep::Config;

fn main() {

    // function to parse cmd line args for config variables
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for `{}`", config.query);
    println!("In file `{}`", config.filename);

    if let Err(e) = fogrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

