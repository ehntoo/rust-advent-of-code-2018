use std::boxed::Box;

#[derive(PartialEq, Debug, Clone)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Box<Node> {
    let numbers: Vec<u32> = input.trim().split(" ").map(|s| s.parse().unwrap()).collect();
    Box::new(node_from_number_iter(&mut numbers.iter()))
}

fn node_from_number_iter<'a, I>(numbers: &mut I) -> Node 
where I: Iterator<Item = &'a u32>,
{
    let num_children = numbers.next().unwrap();
    let num_metadata = numbers.next().unwrap();
    let children = (0 .. *num_children).map(|_| {
        node_from_number_iter(numbers)
    }).collect();
    let metadata = numbers.take(*num_metadata as usize).map(|x| *x).collect();
    Node { children, metadata }
}

#[aoc(day8, part1)]
pub fn solve_part1(node: &Node) -> u32 {
    node.metadata.iter().fold(0, |acc, x| acc + *x) +
        node.children.iter().fold(0, |acc, c| acc + solve_part1(c))
}

#[aoc(day8, part2)]
pub fn solve_part2(node: &Node) -> u32 {
    if node.children.len() == 0 {
        node.metadata.iter().fold(0, |acc, x| acc + *x)
    } else {
        let child_sums: Vec<u32> = node.children.iter().map(|c| solve_part2(c)).collect();
        node.metadata.iter().fold(0, |acc, x| {
            let idx = (*x - 1) as usize;
            acc + if idx < child_sums.len() {
                child_sums[idx]
            } else {
                0
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(input_generator("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"),
        Box::new(Node {
            children: vec![
                Node { children: vec![], metadata: vec![10, 11, 12] },
                Node {
                    children: vec![ Node { children: vec![], metadata: vec![99] }],
                    metadata: vec![2],
                },
            ],
            metadata: vec![1, 1, 2],
        }));
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&*input_generator("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2")), 66);
    }
}
