use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

extern crate regex;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Incorrect number of args");
    let f = File::open(&args[1]).expect("file not found");
    let reader = BufReader::new(&f);

    let mut fabric_map: HashMap<(usize, usize), (String, i32)> = HashMap::new();
    let mut overlaps = 0;
    let mut perfect_claims: HashSet<String> = HashSet::new();
    let claim_re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    for line in reader.lines() {
        // Parse the claim into useful pieces.
        let string = line.unwrap();
        let claim_pieces = claim_re.captures(&string).unwrap();
        let claim = &claim_pieces[1];
        let x: usize = claim_pieces[2].parse().unwrap();
        let y: usize = claim_pieces[3].parse().unwrap();
        let width: usize = claim_pieces[4].parse().unwrap();
        let height: usize = claim_pieces[5].parse().unwrap();

        // We haven't had any overlaps with this claim yet, so initially place it
        // in the perfect_claims set.
        perfect_claims.insert(claim.to_string());
        for row in y..y+height {
            for col in x..x+width {
                let mut entry = fabric_map.entry((row, col)).or_insert((claim.to_string(), 0));
                if entry.1 == 0 {
                    entry.1 = 1;
                } else {
                    if entry.1 == 1 {
                        // This is the first time we've overlapped this segment.
                        overlaps += 1;
                    }
                    perfect_claims.remove(&entry.0);
                    perfect_claims.remove(&claim.to_string());
                    entry.0 = claim.to_string();
                    entry.1 = 2;
                }
            }
        }
    }
    println!("Overlap segments = {}", overlaps);
    for claim in perfect_claims {
        println!("Claim {} does not overlap previous claims", claim);
    }
}