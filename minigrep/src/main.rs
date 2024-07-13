// minigrep is a simple implementation of the standard grep command for finding text in a file
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); // this returns an interator of the CLI arguments passed to minigrep
                                                   // read any command line arguments & collect the values (from the iterator) into a vector
                                                   // calling the collect method on the elements the iterator produces, turns it into a vector
    //dbg!(args);                                    // print the vector using debug macro                 
    let config = Config::new(&args);               // indicating this is associated with the Config struct, so I'm creating an instance of Config
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file"); 
    // fs::read_to_string opens the file in file_path and returns a String of the file's contents

    println!("With text:\n{contents}");
}

// adding this struct because parse_config returns 2 values which are related, and part of one configuration value
struct Config {
    query: String,
    file_path: String,
}

impl Config {   // using an implementation block because it associates the new function with the Config struct
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");     // if user enters wrong amount of arguments
        }
        
        let query = args[1].clone();    // using clone so the String data is copied for the Config instance to own
        let file_path = args[2].clone();

        Config { query, file_path }     // function returns a Config value
    }
}

