use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Incorrect number of args");

    let mut frequencies = HashSet::new();
    let mut first_iter = true;
    let mut found_final = false;
    let mut first_result = 0;
    let mut final_result = 0;
    let mut result = 0;
    while !found_final {
        let mut f = File::open(&args[1]).expect("file not found");
        let mut reader = BufReader::new(&f);
        for line in reader.lines() {
            frequencies.insert(result);
            let line = line.unwrap().to_string();
            let val = line.parse::<i32>().unwrap();
            result += val;
            if frequencies.contains(&result) && !found_final {
                found_final = true;
                final_result = result;
            }
            if found_final && !first_iter {
                break;
            }
        }
        if first_iter {
            first_result = result;
            first_iter = false;
        }
    }
    println!("Result after 1 iteration: {}", first_result);
    println!("First repeated frequency: {}", final_result);
}