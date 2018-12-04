#[derive(PartialEq, Debug, Clone)]
pub struct Nap {
    start: usize,
    end: usize,
    duration: usize,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Entry {
    id: usize,
    asleep: Vec<Nap>,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Entry> {
    let mut sorted_log_entries = input.lines().collect::<Vec<&str>>();
    sorted_log_entries.sort();

    let mut current_guard_id = 0;
    let mut slumber_periods = vec![];
    let mut completed_entries = vec![];
    for e in sorted_log_entries {
        let mut parts = e.split(&['#', ':'][..]);
		// drop the date and hour, since we don't care about those
        parts.next();
        let text_starting_at_minute = parts.next().unwrap();
        let minute = text_starting_at_minute.chars().take(2).collect::<String>()
			.parse::<usize>().unwrap();
        if text_starting_at_minute.len() == 12 {
            // wakes up
            slumber_periods.push(minute);
        } else if text_starting_at_minute.len() == 16 {
            // falls asleep
            slumber_periods.push(minute);
        } else {
            // shift change
            if slumber_periods.len() > 0 {
                completed_entries.push(Entry {
                    id: current_guard_id,
                    asleep: slumber_periods.chunks(2).map(|c| Nap {
                        start: c[0],
                        end: c[1],
                        duration: c[1]-c[0],
                    }).collect()
                });
                slumber_periods.clear();
            }
            current_guard_id = parts.next().unwrap().split(' ').next().unwrap().parse::<usize>().unwrap();
        }
    }
	if slumber_periods.len() > 0 {
		completed_entries.push(Entry {
			id: current_guard_id,
			asleep: slumber_periods.chunks(2).map(|c| Nap {
				start: c[0],
				end: c[1],
				duration: c[1]-c[0],
			}).collect()
		});
		slumber_periods.clear();
	}
    completed_entries
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Entry]) -> usize {
    let mut nap_vec: Vec<usize> = vec![0; 4096];
	for e in input {
        nap_vec[e.id] += e.asleep.iter().map(|n| n.duration).sum::<usize>();
	}
    let best_rested_duration = nap_vec.iter().max().unwrap();
    let best_rested_guard = nap_vec.iter().position(|x| x == best_rested_duration).unwrap();
    // println!("Best rested guard was {} with {} minutes of naps", best_rested_guard, best_rested_duration);

    let mut nap_population_vec: Vec<usize> = vec![0; 60];
    let best_rested_guard_entries = input.iter().filter(|x| x.id == best_rested_guard);
    for e in best_rested_guard_entries {
        for x in e.asleep.clone() {
            for minute in x.start .. x.end {
                nap_population_vec[minute] += 1;
            }
        }
    }

    let sleepy_minute_count = nap_population_vec.iter().max().unwrap();
    let sleepy_minute = nap_population_vec.iter().position(|x| x == sleepy_minute_count).unwrap();
    // println!("Most rested minute was {} with {} minutes of naps", sleepy_minute, sleepy_minute_count);
    best_rested_guard * sleepy_minute
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Entry]) -> usize {
    let mut nap_vec: Vec<Vec<usize>> = vec![vec![0; 60]; 4096];
	for e in input {
        for nap in e.asleep.clone() {
            for i in nap.start .. nap.end {
                nap_vec[e.id][i] += 1;
            }
        }
	}

    let per_guard_same_minute_max: Vec<&usize> = nap_vec.iter().map(|v| v.iter().max().unwrap()).collect();
    let absolute_max = per_guard_same_minute_max.iter().max().unwrap();
    let sleepiest_guard = per_guard_same_minute_max.iter().position(|x| x == absolute_max).unwrap();
    let sleepiest_minute = nap_vec[sleepiest_guard].iter().position(|x| &x == absolute_max).unwrap();
    sleepiest_guard * sleepiest_minute
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(input_generator("[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift"), &[
Entry{id: 10, asleep:
	vec![Nap{start:5, end:25, duration:20}, Nap{start:30, end:55, duration:25}]},
Entry{id: 99, asleep:
	vec![Nap{start:40, end:50, duration:10}]},
]);
    }
}
