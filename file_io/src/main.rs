//use std::env;
use std::fs;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let query = &args[0];
    // let filename = &args[0];

    // println!("Searching for {}", query);
    // --snip--
    println!("In file {}", "poem.txt");

    let contents = fs::read_to_string("poem.txt")
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
