// use itertools::Itertools;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| {
        let mut str = l.chars().collect::<Vec<char>>();
        str.sort();
        str
    }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Vec<char>>) -> i32 {
    let mut double_count = 0;
    let mut triple_count = 0;
    // TODO: figure out how to satisfy the borrow checker here
    // let _grouped_input = input.into_iter().map(|barcode| {
    //     barcode.into_iter().group_by(|&b| b)
    //         .into_iter().map(|(_, g)| g.count())
    // });

    for char_vec in input {
        let mut last_seen_char = 'A';
        let mut char_count = 0;
        let mut not_seen_double = true;
        let mut not_seen_triple = true;
        for c in char_vec {
            if last_seen_char == *c {
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
                last_seen_char = *c;
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
}
