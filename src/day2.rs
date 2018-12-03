// use itertools::Itertools;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.trim().to_string()).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<String>) -> i32 {
    let mut double_count = 0;
    let mut triple_count = 0;
    // TODO: figure out how to satisfy the borrow checker here
    // let _grouped_input = input.into_iter().map(|barcode| {
    //     barcode.into_iter().group_by(|&b| b)
    //         .into_iter().map(|(_, g)| g.count())
    // });
    // let sorted_vec = input.to_vec();

    let sorted_vec: Vec<Vec<char>> = input.iter().map(|l| {
        let mut str = l.chars().collect::<Vec<char>>();
        str.sort();
        str
    }).collect();

    for char_vec in sorted_vec {
        let mut last_seen_char = 'A';
        let mut char_count = 0;
        let mut not_seen_double = true;
        let mut not_seen_triple = true;
        for c in char_vec {
            if last_seen_char == c {
                char_count += 1;
            } else {
                if char_count == 2 && not_seen_double {
                    double_count += 1;
                    not_seen_double = false;
                }
                if char_count == 3 && not_seen_triple {
                    triple_count += 1;
                    not_seen_triple = false;
                }
                char_count = 1;
                last_seen_char = c;
            }
        }
        if char_count == 2 && not_seen_double {
            double_count += 1;
        }
        if char_count == 3 && not_seen_triple {
            triple_count += 1;
        }
    }
    double_count * triple_count
}

fn compute_diff(s1: &str, s2: &str) -> Vec<char> {
    let mut same_chars = Vec::new();
    let s2_chars: Vec<char> = s2.chars().collect();
    for (i, c) in s1.chars().enumerate() {
        if s2_chars[i] == c {
            same_chars.push(c);
        }
    }
    same_chars
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<String>) -> String {
    let goal_len = input[0].len();
    let mut local_vec = input.to_vec();
    local_vec.sort_unstable();

    for i in 0..local_vec.len() {
        for j in 0..local_vec.len() {
            if i == j { continue }
            let same_chars = compute_diff(&local_vec[i], &local_vec[j]);
            if same_chars.len() == goal_len - 1 {
                return same_chars.iter().collect()
            }
        }
    }
    "hi".to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(solve_part1(&input_generator("abcdef")), 0);
        assert_eq!(solve_part1(&input_generator("bababc")), 1);
        assert_eq!(solve_part1(&input_generator("abbcde")), 0);
        assert_eq!(solve_part1(&input_generator("abcccd")), 0);
        assert_eq!(solve_part1(&input_generator("aabcdd")), 0);
        assert_eq!(solve_part1(&input_generator("abcdee")), 0);
        assert_eq!(solve_part1(&input_generator("ababab")), 0);

        assert_eq!(solve_part1(&input_generator("bababc\nabcdef")), 1);
        assert_eq!(solve_part1(&input_generator("bababc\nbababc")), 4);
        assert_eq!(solve_part1(&input_generator("bababc\nabbcde")), 2);
        assert_eq!(solve_part1(&input_generator("bababc\nabcccd")), 2);
        assert_eq!(solve_part1(&input_generator("bababc\naabcdd")), 2);
        assert_eq!(solve_part1(&input_generator("bababc\nabcdee")), 2);
        assert_eq!(solve_part1(&input_generator("bababc\nababab")), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part2(&input_generator("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz")), "fgij");
    }
}
