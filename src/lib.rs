mod binary_tree;
mod node;

use crate::binary_tree::BinaryTree;
use crate::node::Stringify;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process;

pub struct Config {
    pub filename: String,
    pub number_of_nodes: u32,
}

impl Config {
    pub fn new(args: Vec<String>) -> Self {
        if args.len() < 2 {
            eprintln!("Error: not enough arguments supplied");
            print_usage();
            process::exit(1);
        }

        let number_of_nodes = match args[1].parse::<u32>() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Error: cannot parse number_of_nodes: argument must be convertable to u32 number");
                print_usage();
                process::exit(1);
            }
        };

        Config {
            filename: args[0].clone(),
            number_of_nodes,
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let filename = Path::new(&config.filename);
    let _file = File::create(&filename);

    let stringified_data = create_json_content(config.number_of_nodes);
    fs::write(&filename, stringified_data).expect("Couldn't write JSON to target file");

    Ok(())
}

fn create_json_content(iterations: u32) -> String {
    let mut data = Vec::new();
    data.push(("[").to_string());

    let mut tree = BinaryTree::init("Test 1".to_string());
    data.push(tree.get_node_object().to_json());

    for i in 1..iterations {
        data.push((",").to_string());
        let new_node = tree.insert(i.to_string());
        data.push(new_node.as_ref().unwrap().get_node_object().to_json());
    }

    data.push(("]").to_string());
    data.into_iter().map(String::from).collect()
}

fn print_usage() {
    eprintln!("Usage: cargo run <TARGET_FILE> <number_of_nodes>")
}
