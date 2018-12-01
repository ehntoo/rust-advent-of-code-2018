use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    let mut observed_sums = HashSet::new();
    let mut freq = 0;
    'outer: loop {
        for i in input {
            freq = freq + i;
            if observed_sums.contains(&freq) {
                break 'outer;
            }
            observed_sums.insert(freq);
        }
    }
    freq
}
