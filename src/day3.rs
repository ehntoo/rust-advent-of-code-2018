use bit_vec::BitVec;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Claim> {
    input.lines().map(|l| {
        let mut parts = l.split(&['#', '@', ',', ':', 'x'][..]);
        // Skip the first empty '#' entry
        parts.next();
        let nums: Vec<u32> = parts.map(|p| p.trim().parse::<u32>().unwrap()).collect();
        Claim {
            id:     nums[0],
            left:   nums[1],
            top:    nums[2],
            width:  nums[3],
            height: nums[4],
        }
    }).collect()
}

fn get_overclaimed_map(input: &[Claim]) -> BitVec {
    // create two bitmaps, coloring in the first with each claim as we encounter it and the second
    // as soon as we encounter a cell in the first that's already been marked
    //
    // to answer the question "How many square inches of fabric are within two or more claims?", we
    // can just do a population count on the second bitmap.
    // hey, there's this handy bit-vec crate.
    let mut claimed = BitVec::from_elem(1_000 * 1_000, false);
    let mut over_claimed = BitVec::from_elem(1_000 * 1_000, false);
    for claim in input {
        for top_idx in claim.top .. (claim.top+claim.height) {
            for left_idx in claim.left .. (claim.left+claim.width) {
                let cur_flat_idx: usize = (left_idx + (1_000 * top_idx)) as usize;
                if !claimed.get(cur_flat_idx).unwrap() {
                    claimed.set(cur_flat_idx, true);
                } else {
                    // println!("Over-claiming! left_idx {} top_idx {} flat_idx {}", left_idx, top_idx, cur_flat_idx);
                    over_claimed.set(cur_flat_idx, true);
                }
            }
        }
    }
    over_claimed
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Claim]) -> u32 {
    let over_claimed = get_overclaimed_map(input);
    over_claimed.iter().filter(|b| *b).count() as u32
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Claim]) -> i32 {
    let over_claimed = get_overclaimed_map(input);
    'next_claim: for claim in input {
        for top_idx in claim.top .. (claim.top+claim.height) {
            for left_idx in claim.left .. (claim.left+claim.width) {
                let cur_flat_idx: usize = (left_idx + (1_000 * top_idx)) as usize;
                if over_claimed.get(cur_flat_idx).unwrap() {
                    continue 'next_claim;
                }
            }
        }
        return claim.id as i32;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(input_generator("#1 @ 527,351: 24x10")[0], Claim {
            id: 1,
            left: 527,
            top: 351,
            width: 24,
            height: 10,
        });
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&input_generator("#1 @ 1,3: 4x4")), 0);
        assert_eq!(solve_part1(&input_generator("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2")), 4);
    }
}
