mod json;

use crate::json::create_json_content;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

pub struct Config {
    pub filename: String,
    pub number_of_nodes: u32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("You need to enter at least one argument (filename)");
        }
        let filename = args[1].clone();
        let number_of_nodes = match args[2].clone().parse::<u32>() {
            Ok(number) => number,
            Err(_e) => return Err("Argument for number of nodes must be convertable to a number and lower than max value of u32 integer type"),
        };

        Ok(Config {
            filename,
            number_of_nodes,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let filename = Path::new(&config.filename);
    let _file = File::create(&filename);

    let stringified_data = create_json_content(config.number_of_nodes);
    fs::write(&filename, stringified_data).expect("Couldn'write json to file");

    Ok(())
}

