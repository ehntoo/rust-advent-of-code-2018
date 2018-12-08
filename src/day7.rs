use std::collections::HashSet;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub struct Requirement {
    step_id: char,
    requires: char,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Requirement> {
    input.lines().map(|l| {
        let mut parts = l.trim().split(" must be finished before step ");
        let requires = parts.next().unwrap().chars().skip(5).next().unwrap();
        let step_id = parts.next().unwrap().chars().next().unwrap();
        Requirement { step_id, requires }
    }).collect()
}

fn get_next_step(steps: &HashMap<char, HashSet<char>>) -> char {
    let mut remaining_steps = steps.keys().collect::<Vec<&char>>();
    remaining_steps.sort_unstable();
    **remaining_steps.iter().find(|s| steps.get(s).unwrap().is_empty()).unwrap()
}

#[aoc(day7, part1)]
pub fn solve_part1(requirements: &[Requirement]) -> String {
    let mut steps_with_requirements: HashMap<char, HashSet<char>> = HashMap::new();
    // initialize all the characters to get them in there
    for c in b'A'..=b'Z' {
        let s = c as char;
        steps_with_requirements.insert(s, HashSet::new());
    }
    for c in b'A'..=b'Z' {
        let s = c as char;
        for required_by in requirements.iter().filter(|&r| r.requires == s) {
            let entry = steps_with_requirements.entry(required_by.step_id).or_insert(HashSet::new());
            entry.insert(s);
        }
    }

    let mut output_chars: Vec<char> = vec![];

    while !steps_with_requirements.is_empty() {
        let next_step = get_next_step(&steps_with_requirements);
        // println!("Got next step: {:?}", next_step);
        output_chars.push(next_step);

        for (_, s) in steps_with_requirements.iter_mut() {
            s.remove(&next_step);
        }
        steps_with_requirements.remove(&next_step);
    }

    output_chars.iter().map(|x| *x).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(input_generator("Step A must be finished before step L can begin."),
        vec![Requirement {
            step_id: 'L',
            requires: 'A'
        }]);
    }
}
