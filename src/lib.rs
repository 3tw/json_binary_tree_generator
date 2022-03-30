use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

pub struct Config {
    pub filename: String,
    pub number_of_objects: u16,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("You need to enter at least one argument (filename)");
        }
        let filename = args[1].clone();
        let number_of_objects = match args[2].clone().parse::<u16>() {
            Ok(number) => number,
            Err(_e) => return Err("Argument for number of object must be convertable to a number"),
        };

        Ok(Config {
            filename,
            number_of_objects,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let filename = Path::new(&config.filename);
    let file = File::create(&filename);

    // DEV
    let data = "Random data";
    fs::write(&filename, &data);

    println!("file: {:?}", file);

    Ok(())
}
