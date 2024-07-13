// minigrep is a simple implementation of the standard grep command for finding text in a file
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // this returns an interator of the CLI arguments passed to minigrep
                                                   // read any command line arguments & collect the values (from the iterator) into a vector
                                                   // calling the collect method on the elements the iterator produces, turns it into a vector
    //dbg!(args);                                    // print the vector using debug macro                 
    let query = &args[1];                          // saving the values of the two arguments in variables so they can be used again
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}
