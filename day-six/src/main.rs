use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::vec::Vec;

/*
For each input coord:
     - Add coord to set
     - 
*/
fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let string = line.unwrap().to_string();
        println!("{}", string);
    }
}
