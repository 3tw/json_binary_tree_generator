mod json_objects;

use crate::json_objects::Edge;
// use crate::json_objects::JsonData;
use crate::json_objects::Node;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

pub struct Config {
    pub filename: String,
    pub number_of_nodes: u16,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("You need to enter at least one argument (filename)");
        }
        let filename = args[1].clone();
        let number_of_nodes = match args[2].clone().parse::<u16>() {
            Ok(number) => number,
            Err(_e) => return Err("Argument for number of nodes must be convertable to a number"),
        };

        Ok(Config {
            filename,
            number_of_nodes,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let filename = Path::new(&config.filename);
    let file = File::create(&filename);

    let mut data = Vec::new();

    let node = Node::new().expect("Couldn't serialize data to json");
    let edge = Edge::new().expect("Couldn't serialize data to json");
    data.push(String::from("{\"data\":["));
    data.push(node);
    data.push(String::from(","));
    data.push(edge);
    data.push(String::from("]}"));

    let stringified_data: String = data.into_iter().map(String::from).collect();
    fs::write(&filename, stringified_data).expect("Couldn'write json to file");

    println!("file: {:?}", file);

    Ok(())
}
