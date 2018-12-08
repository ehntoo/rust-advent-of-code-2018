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

fn get_requirement_map(requirements: &[Requirement]) -> HashMap<char, HashSet<char>> {
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

    steps_with_requirements
}

#[aoc(day7, part1)]
pub fn solve_part1(requirements: &[Requirement]) -> String {
    let mut steps_with_requirements = get_requirement_map(requirements);
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

fn available_steps(steps: &HashMap<char, HashSet<char>>) -> Vec<char> {
    let mut remaining_steps = steps.keys().collect::<Vec<&char>>();
    remaining_steps.sort_unstable();
    remaining_steps.iter().filter(|s| steps.get(s).unwrap().is_empty())
        .map(|x| **x).collect()
}

#[aoc(day7, part2)]
pub fn solve_part2(requirements: &[Requirement]) -> u32 {
    const NUM_WORKERS: usize = 5;
    const STEP_DURATION: u32 = 60;
    let mut steps_with_requirements = get_requirement_map(requirements);

    let mut current_time = 0;

    #[derive(PartialEq, Debug, Clone)]
    struct StepInWork {
        step_id: char,
        completion_time: u32,
    }

    let mut steps_in_work: Vec<StepInWork> = vec![];
    // let mut work_completion_times: HashSet<StepInWork> = HashSet::new();

    while !steps_with_requirements.is_empty() {
        if steps_in_work.len() > NUM_WORKERS {
            panic!("Too much working going on.");
        }
        // println!("starting a new time increment");
        let mut w = 0;
        while w != steps_in_work.len() {
            if steps_in_work[w].completion_time < current_time {
                panic!("this should have already been removed");
            }
            if steps_in_work[w].completion_time == current_time {
                // println!("Finishing {:?}", steps_in_work[w]);
                let completed_step = steps_in_work.remove(w);

                for (_, s2) in steps_with_requirements.iter_mut() {
                    s2.remove(&completed_step.step_id);
                }
                steps_with_requirements.remove(&completed_step.step_id);
            } else {
                w += 1;
            }
        }

        let available_steps = available_steps(&steps_with_requirements);
        for s in available_steps.iter().take(NUM_WORKERS) {
            match steps_in_work.iter().find(|w| w.step_id == *s) {
                Some(_) => { },
                None => {
                    let completion_time = current_time + STEP_DURATION + (*s as u8 - b'A' + 1) as u32;
                    let next_job = StepInWork{step_id: *s, completion_time};
                    // println!("time: {:?}, scheduling job: {:?}", current_time, next_job);
                    steps_in_work.push(next_job);
                }
            }
        }

        // increment time to the next step in steps_in_work
        current_time = steps_in_work.iter().min_by_key(|s| s.completion_time)
            .unwrap_or(&StepInWork{step_id: 'a', completion_time: current_time})
            .completion_time;
    }
    if steps_in_work.is_empty() {
        current_time
    } else {
        steps_in_work.iter().max_by_key(|w| w.completion_time).unwrap().completion_time
    }
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
