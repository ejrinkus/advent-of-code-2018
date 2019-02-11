extern crate plane;

use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::vec::Vec;
use plane::Location;
use plane::Plane;

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(f);
    let mut plane = Plane::new();
    for line in reader.lines() {
        let string = line.unwrap();
        let pieces: Vec<&str> = string.split(", ").collect();
        let loc = Location {
            x: pieces[0].parse().unwrap(),
            y: pieces[1].parse().unwrap(),
            reach: -1,
            expansions: Vec::new(),
        };
        plane.add_location(loc);
    }
    let largest = plane.get_largest_reach().unwrap();
    println!("Largest reach from {}, {}", largest.x, largest.y);
}
