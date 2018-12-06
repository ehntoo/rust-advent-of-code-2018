#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<u8> {
    input.trim().as_bytes().to_vec()
}

fn reduce_polymer(polymer: &[u8]) -> Vec<u8> {
    let mut found_change = true;
    let mut local_vec: Vec<u8> = polymer.to_vec();
    while found_change {
        found_change = false;
        let mut new_vec: Vec<u8> = vec![];

        let mut iter = local_vec.windows(2);
        loop {
            match iter.next() {
                Some(x) => {
                    // println!("Processing {} and {}", x[0], x[1]);
                    if x[0] == x[1] + 0x20 || x[0] == x[1] - 0x20 {
                        found_change = true;
                        // consume both this and the next value
                        iter.next();
                    } else {
                        new_vec.push(x[0]);
                    }
                },
                None => {
                    break
                }
            }
        }

        let vec_last = *local_vec.iter().rev().next().unwrap();
        let vec_next_to_last = *local_vec.iter().rev().nth(1).unwrap();
        if vec_last != vec_next_to_last + 0x20 && vec_last != vec_next_to_last - 0x20  {
            // println!("pushing the edge case");
            new_vec.push(vec_last);
        }

        local_vec = new_vec;

        if local_vec.len() < 2 {
            break
        }
    }
    // println!("got string: {}", std::str::from_utf8(&local_vec).unwrap());
    local_vec
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[u8]) -> usize {
    reduce_polymer(input).len()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[u8]) -> usize {
    let input_vec = input.to_vec();
    (0x41u8..0x5a).map(|a| {
        let filtered_input: Vec<u8> = input_vec.iter().filter(|c| **c != a && **c != a + 0x20)
            .map(|c| *c).collect();
        reduce_polymer(&filtered_input).len()
    }).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(solve_part1(&input_generator("aA")), "".len());
        assert_eq!(solve_part1(&input_generator("aAaA")), "".len());
        assert_eq!(solve_part1(&input_generator("abBA")), "".len());
        assert_eq!(solve_part1(&input_generator("abAB")), "abAB".len());
        assert_eq!(solve_part1(&input_generator("aabAAB")), "aabAAB".len());
        assert_eq!(solve_part1(&input_generator("aabBAAB")), "B".len());
        assert_eq!(solve_part1(&input_generator("dabAcCaCBAcCcaDA")), "dabCBAcaDA".len());
        assert_eq!(solve_part1(&input_generator("hHsSmMHhhHwWlLoyYCcOZz")), "".len());
    }
}
