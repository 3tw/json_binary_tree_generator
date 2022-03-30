use json_generator::Config;
use std::env;
use std::process;

fn main() {
    // Gets arguments from command line
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("Filename: {}", config.filename);
    println!("Number of objects: {}", config.number_of_objects);

    if let Err(e) = json_generator::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
