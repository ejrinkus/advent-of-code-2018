use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::vec::Vec;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Incorrect number of args");
    let f = File::open(&args[1]).expect("file not found");
    let mut reader = BufReader::new(&f);

    // Input files for this problem should only have one polymer (i.e. one line).
    let mut poly_string = String::new();
    reader.read_line(&mut poly_string).expect("Unable to read line");

    let polymer = poly_string.into_bytes();
    let mut activated_polymer: Vec<i8> = Vec::new();

    // Loop over each unit in the polymer (use indices since we don't want polymer to be moved into this loop).
    for i in 0..polymer.len() {
        // Pop the last unit from the activated polymer for comparison
        let prev_unit = activated_polymer.pop();

        // If it's None, then the activated polymer is currently empty. So push on the
        // current unit and move on.
        if prev_unit.is_none() {
            activated_polymer.push(polymer[i] as i8);
            continue;
        }

        // Otherwise, we need to get the current unit in the unactivated polymer for
        // comparison (and unwrap the previous unit, because we now know it isn't None).
        let prev_unit = prev_unit.unwrap();
        let curr_unit = polymer[i] as i8;
        let diff = curr_unit - prev_unit;

        // The distance between a lowercase letter and its capital is always 32 for
        // ascii characters. Despite Rust using unicode strings, input polymers will
        // always be ascii values, which are equivalent between unicode and ordinary ascii.
        if diff.abs() == 32 {
            // The current pair gets destroyed. We've already popped the previous value,
            // so we can just skip the current value.
            continue;
        }

        // No destruction, so we can push the current value.
        activated_polymer.push(prev_unit);
        activated_polymer.push(curr_unit);
    }
    println!("Orignal polymer had {} units, activated polymer has {} remaining", polymer.len(), activated_polymer.len());

    // Now that the original polymer is activated, we can try filtering out specific letters and recompressing.
    // To do so, wrap the same logic above in another loop that loops over each letter of the alphabet, and add
    // an extra check to the compression loop to skip the current letter in the alphabet.
    let mut min_length = activated_polymer.len();
    let mut min_char = '\0';
    for c in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
              'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'].to_vec() {
        let mut reactivated_polymer: Vec<i8> = Vec::new();
        for i in 0..activated_polymer.len() {
            // New check: skip any instances of c (or the capital version of c)
            let lower = c as i8;
            let upper = lower - 32;
            if activated_polymer[i] == lower || activated_polymer[i] == upper {
                continue;
            }
            // Remaining logic is a copy of the loop above.
            let prev_unit = reactivated_polymer.pop();
            if prev_unit.is_none() {
                reactivated_polymer.push(activated_polymer[i]);
                continue;
            }
            let prev_unit = prev_unit.unwrap();
            let curr_unit = activated_polymer[i];
            let diff = curr_unit - prev_unit;
            if diff.abs() == 32 {
                continue;
            }
            reactivated_polymer.push(prev_unit);
            reactivated_polymer.push(curr_unit);
        }
        println!("Filtering out {} and recompressing resulted in {} units", c, reactivated_polymer.len());
        if reactivated_polymer.len() < min_length {
            min_length = reactivated_polymer.len();
            min_char = c;
        }
    }
    println!("The optimal character to filter out was {}, and resulted in a new length of {}", min_char, min_length);
}