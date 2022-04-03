mod json;

use crate::json::create_json_content;

use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

pub struct Config {
    pub filename: String,
    pub number_of_nodes: u32,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let number_of_nodes = match args.next() {
            Some(arg) => arg
                .parse::<u32>()
                .expect("Cannot parse number_of_nodes to u32"),
            None => {
                println!("Default number of nodes set to 10");
                10
            }
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
