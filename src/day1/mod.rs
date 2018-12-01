use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

pub fn solve() {
    let file = File::open("src/day1/input.txt").unwrap();
    let lines: Vec<i32> = BufReader::new(file).lines().map(|l| l.unwrap().parse::<i32>().unwrap()).collect();
    let sum = lines.iter().fold(0, |acc, x| acc + x);
    println!("resulting frequency was: {}", sum);

    // part 2
    // need to find the earliest frequency that is reached twice
    // well, let's produce a list of the resulting frequency after application rather than just
    // producing the final output.
    let mut observed_sums = HashSet::new();
    let mut freq = 0;
    let mut loops = 0;
    'outer: loop {
        for i in &lines {
            freq = freq + i;
            if observed_sums.contains(&freq) {
                break 'outer;
            }
            observed_sums.insert(freq);
        }
        loops += 1;
    }
    println!("repeated frequency was: {}", freq);
    println!("found in {} loops", loops);
}
