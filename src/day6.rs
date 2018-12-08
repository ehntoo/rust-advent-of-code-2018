use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    right: usize,
    down: usize,
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Point> {
    input.lines().map(|l| {
        let mut parts = l.trim().split(", ");
        Point {
            right: parts.next().unwrap().parse().unwrap(),
            down: parts.next().unwrap().parse().unwrap(),
        }
    }).collect()
}

fn compute_distance(point_one: &Point, point_two: &Point) -> usize {
    let right_distance = if point_one.right > point_two.right {
        point_one.right - point_two.right
    } else {
        point_two.right - point_one.right
    };
    let down_distance = if point_one.down > point_two.down {
        point_one.down - point_two.down
    } else {
        point_two.down - point_one.down
    };
    right_distance + down_distance
}

// strategy - find a bounding box for coordinates, since any points further
// east/south will either be part of an infinite region or tied.
//
// then, iterate over each point within the bounding box and compute distance
// for each known point, keeping a record of which known point has the shortest
// distance.
//
// to solve, we can then just sum up each point's closest noted regions.
//
// hm. I was overlooking the infinite regions. there's a few paths forward here
// 1 - shift things further into the grid and add a buffer to the bounding box,
// then trim everything above a certain bound which would be unachievable
// without an "infinite" square.
// 2 - iterate over the bounding box coordinates, and eliminate each of the
// regions which are winning any of them
#[aoc(day6, part1)]
pub fn solve_part1(points: &[Point]) -> u32 {
    let bound_right = points.iter().max_by_key(|p| p.right).unwrap();
    let bound_down = points.iter().max_by_key(|p| p.down).unwrap();
    println!("bounding box: {:?}, {:?}", bound_right.right, bound_down.down);

    let mut closest_points = vec![vec![std::usize::MAX; bound_right.right+1]; bound_down.down+1];

    for right in 0 .. bound_right.right+1 {
        for down in 0 .. bound_down.down+1 {
            let mut current_min_distance = std::usize::MAX;
            for (i, p) in points.iter().enumerate() {
                let distance = compute_distance(p, &Point{right, down});
                if distance < current_min_distance {
                    current_min_distance = distance;
                    closest_points[down][right] = i;
                } else if distance == current_min_distance {
                    closest_points[down][right] = std::usize::MAX;
                }
            }
        }
    }

    let mut invalidated_regions = HashSet::new();
    for row in 0 .. bound_down.down+1 {
        invalidated_regions.insert(closest_points[row][0]);
        invalidated_regions.insert(closest_points[row][bound_right.right]);
    }

    for column in 0 .. bound_right.right+1 {
        invalidated_regions.insert(closest_points[0][column]);
        invalidated_regions.insert(closest_points[bound_down.down][column]);
    }

    let mut point_totals = vec![0; points.len()];
    for row in closest_points {
        for c in row {
            if c != std::usize::MAX && !invalidated_regions.contains(&c){
                point_totals[c] += 1;
            }
        }
    }
    println!("wound up with point totals: {:?}", point_totals);
    *point_totals.iter().max().unwrap()
}

#[aoc(day6, part2)]
pub fn solve_part2(points: &[Point]) -> u32 {
    const DISTANCE_THRESHOLD: usize = 10_000;
    let bound_right = points.iter().max_by_key(|p| p.right).unwrap();
    let bound_down = points.iter().max_by_key(|p| p.down).unwrap();

    let mut safe_total = 0;

    for right in 0 .. bound_right.right+1 {
        for down in 0 .. bound_down.down+1 {
            let distance_sum = points.iter().fold(0, |acc, p| {
                acc + compute_distance(p, &Point{right, down})
            });
            if distance_sum < DISTANCE_THRESHOLD {
                safe_total += 1;
            }
        }
    }

    safe_total
}
