use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); // init vector to hold cmd line args

    // store relavant args in variables
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for `{}`", query);
    println!("In file `{}`", filename);

    // store file contents in variable using fs function
    let contents = fs::read_to_string(filename)
        .expect("Error in reading file");

    println!("With text:\n{}", contents);
}
