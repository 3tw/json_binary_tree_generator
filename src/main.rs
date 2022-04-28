use json_generator::Config;
use std::env;
use std::process;

fn main() {
    // Gets arguments from command line
    let config = Config::new(env::args().skip(1).collect());

    println!("Filename: {}", config.filename);
    println!("Number of objects: {}", config.number_of_nodes);

    if let Err(e) = json_generator::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
