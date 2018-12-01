use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let file = File::open("src/day1/input.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap().parse::<i32>().unwrap());
    // part 2
    // need to find the earliest frequency that is reached twice
    // well, let's produce a list of the resulting frequency after application rather than just
    // producing the final output.
    let sums = lines.fold(vec![0], |mut acc, x| {
        acc.push(acc.last().unwrap() + x);
        acc
    });
    println!("resulting frequency was: {}", sums.last().unwrap());
}
