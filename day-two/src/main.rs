use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

extern crate trie;
use trie::Trie;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Incorrect number of args");
    let f = File::open(&args[1]).expect("file not found");
    let reader = BufReader::new(&f);

    let mut double_count = 0;
    let mut triple_count = 0;
    let mut trie = Trie::new();
    let mut maybe_match = None;
    for line in reader.lines() {
        let string = line.unwrap().to_string();

        // Part 1
        let mut map = HashMap::new();
        for c in string.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        let mut found_double = false;
        let mut found_triple = false;
        for (_, count) in map {
            if count == 2 {
                found_double = true;
            }
            if count == 3 {
                found_triple = true;
            }
        }
        if found_double {
            double_count += 1;
        }
        if found_triple {
            triple_count += 1;
        }

        // Part 2
        if maybe_match.is_none() {
            maybe_match = trie.match_off_by_one(&string);
            trie.insert(string);
        }
        // We don't need to keep adding more strings if we already found the match.
    }
    println!("Doubles: {}, Triples: {}, Checksum: {}",
             double_count, triple_count, double_count*triple_count);
    match maybe_match {
        Some(string) => println!("Found off by one: {}", string),
        None => println!("Did not find off by one")
    }
}