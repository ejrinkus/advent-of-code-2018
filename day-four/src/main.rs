use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::vec::Vec;

extern crate regex;
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
enum Action {
    Start,
    Sleep,
    Wake
}

struct SleepTime {
    total: i32,
    per_minute: [i32; 60]
}

#[derive(Debug)]
struct GuardEntry {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    guard: String,
    action: Action
}

impl GuardEntry {
    fn new() -> GuardEntry {
        return GuardEntry {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            guard: String::new(),
            action: Action::Start
        }
    }
}

impl PartialEq for GuardEntry {
    fn eq(&self, other: &GuardEntry) -> bool {
        return self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.hour == other.hour
            && self.minute == other.minute;
    }
}
impl Eq for GuardEntry {}
impl Ord for GuardEntry {
    fn cmp(&self, other: &GuardEntry) -> Ordering {
        let year_order = self.year.cmp(&other.year);
        if year_order != Ordering::Equal {
            return year_order;
        }
        let month_order = self.month.cmp(&other.month);
        if month_order != Ordering::Equal {
            return month_order;
        }
        let day_order = self.day.cmp(&other.day);
        if day_order != Ordering::Equal {
            return day_order;
        }
        let hour_order = self.hour.cmp(&other.hour);
        if hour_order != Ordering::Equal {
            return hour_order;
        }
        return self.minute.cmp(&other.minute);
    }
}
impl PartialOrd for GuardEntry {
    fn partial_cmp(&self, other: &GuardEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn to_action(action: &str) -> Option<Action> {
    if action == "begins shift" {
        return Some(Action::Start);
    }
    if action == "falls asleep" {
        return Some(Action::Sleep);
    }
    if action == "wakes up" {
        return Some(Action::Wake);
    }
    return None;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Incorrect number of args");
    let f = File::open(&args[1]).expect("file not found");
    let reader = BufReader::new(&f);

    // Parse the input into a Vec of the entries.
    let mut guard_entries: Vec<GuardEntry> = Vec::new();
    let re = Regex::new(r"^\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\](?: Guard #(\d+))? (.*)$").unwrap();
    for line in reader.lines() {
        let string = line.unwrap();
        let pieces = re.captures(&string).unwrap();

        let mut entry = GuardEntry {
            year: pieces[1].parse().unwrap(),
            month: pieces[2].parse().unwrap(),
            day: pieces[3].parse().unwrap(),
            hour: pieces[4].parse().unwrap(),
            minute: pieces[5].parse().unwrap(),
            guard: String::new(),
            action: to_action(&pieces[7]).unwrap()
        };
        let id = pieces.get(6);
        if id.is_some() {
            // Only entries with action "Start" have an explicit guard id.
            entry.guard = String::from(id.unwrap().as_str());
        }
        guard_entries.push(entry);
    }

    // Use built-in sort to sort the entries.
    guard_entries.sort_unstable();

    // Iterate over the now in-order entries and build out a mapping of guard IDs to SleepTimes.
    // Also keep track of the guard with the (currently) largest sleep total, the previous entry, and
    // the currently active guard.
    let mut sleep_totals: HashMap<String, SleepTime> = HashMap::new();
    let mut prev = GuardEntry::new();
    let mut active_guard = String::new();
    // (guard id, total sleep for that guard, longest minute)
    let mut max_guard = (String::new(), 0, 0);
    // (minute, total for that minute, guard id)
    let mut max_minute = (0, 0, String::new());
    for entry in guard_entries {
        // Populate the active_guard if this is the start of the shift.
        if entry.action == Action::Start {
            active_guard = entry.guard.clone();
        }
        // If this is a wake action, add the sleep information to the sleep_totals map.
        else if prev.action == Action::Sleep && entry.action == Action::Wake {
            let mins_asleep = entry.minute - prev.minute;
            let mut sleep_time = sleep_totals.entry(active_guard.clone()).or_insert(SleepTime {
                total: 0,
                per_minute: [0; 60]
            });
            sleep_time.total += mins_asleep;
            // Check to see if we have a new max guard.
            if max_guard.1 < sleep_time.total {
                max_guard.1 = sleep_time.total;
                max_guard.0 = active_guard.clone();
            }
            // Update the max minute for this guard.
            for min in prev.minute..entry.minute {
                sleep_time.per_minute[min as usize] += 1;
                if sleep_time.per_minute[min as usize] > sleep_time.per_minute[max_guard.2 as usize]
                    && active_guard == max_guard.0 {
                    // Also update the max minute for this guard if they are also the max guard.
                    max_guard.2 = min;
                }
                if sleep_time.per_minute[min as usize] > max_minute.1 {
                    max_minute.0 = min;
                    max_minute.1 = sleep_time.per_minute[min as usize];
                    max_minute.2 = active_guard.clone();
                }
            }
        }

        // Store this entry for reference in the next iteration.
        prev = entry;
    }
    println!("Guard {} slept the most ({} minutes), and they slept most frequently during minute {}",
             max_guard.0, max_guard.1, max_guard.2);
    println!("The guard that slept the most on a specific minute was guard {}, on minute {}",
             max_minute.2, max_minute.0);
}