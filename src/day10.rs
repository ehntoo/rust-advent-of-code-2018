#[derive(PartialEq, Debug, Clone)]
pub struct PointOfLight {
    x: i32,
    y: i32,
    vel_x: i32,
    vel_y: i32,
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<PointOfLight> {
    input.lines().map(|l| {
        let line = l.trim();
        let x = line.chars().skip(10).take(6).collect::<String>().trim().parse().unwrap();
        let y = line.chars().skip(18).take(6).collect::<String>().trim().parse().unwrap();
        let vel_x = line.chars().skip(36).take(2).collect::<String>().trim().parse().unwrap();
        let vel_y = line.chars().skip(40).take(2).collect::<String>().trim().parse().unwrap();
        PointOfLight { x, y, vel_x, vel_y }
    }).collect()
}

#[derive(PartialEq, Debug, Clone)]
pub struct BoundingBox {
    x_min: i32,
    y_min: i32,
    x_max: i32,
    y_max: i32,
}

fn predict_point_at_timestep(point: &PointOfLight, ts: u32) -> PointOfLight {
    PointOfLight {
        vel_x: point.vel_x,
        vel_y: point.vel_y,
        x: point.x + point.vel_x * ts as i32,
        y: point.y + point.vel_y * ts as i32,
    }
}

fn bounding_box_at_timestep(points: &[PointOfLight], ts: u32) -> BoundingBox {
    let mut bounding_box = BoundingBox {
        x_min: std::i32::MAX,
        y_min: std::i32::MAX,
        y_max: std::i32::MIN,
        x_max: std::i32::MIN,
    };
    for p in points {
        let fp = predict_point_at_timestep(&p, ts);
        if fp.x < bounding_box.x_min { bounding_box.x_min = fp.x }
        if fp.y < bounding_box.y_min { bounding_box.y_min = fp.y }
        if fp.x > bounding_box.x_max { bounding_box.x_max = fp.x }
        if fp.y > bounding_box.y_max { bounding_box.y_max = fp.y }
    }
    bounding_box
}

fn print_starfield(points: &[PointOfLight]) -> () {
    let bounds = bounding_box_at_timestep(points, 0);
    let mut starfield_vecs = vec![vec![false; (bounds.x_max - bounds.x_min) as usize + 1]; (bounds.y_max - bounds.y_min) as usize + 1];
    for p in points {
        let x = if bounds.x_min < 0 { p.x + bounds.x_min } else { p.x - bounds.x_min };
        let y = if bounds.y_min < 0 { p.y + bounds.y_min } else { p.y - bounds.y_min };
        starfield_vecs[y as usize][x as usize] = true;
    }
    for x in starfield_vecs {
        let printable_vec: String = x.iter().map(|v| if *v {'x'} else {' '}).collect();
        println!("{:?}", printable_vec);
    }
}

#[aoc(day10, part1)]
pub fn solve_part1(points: &[PointOfLight]) -> u32 {
    let smallest_area = (0..100000).map(|ts| {
        let bounds = bounding_box_at_timestep(points, ts);
        let area = (bounds.x_max - bounds.x_min) as i64 * (bounds.y_max - bounds.y_min) as i64;
        (ts, area)
    }).min_by_key(|x| x.1).expect("couldn't find the minimum area");
    println!("Found a star field of area {:?} at ts={:?}", smallest_area.1, smallest_area.0);

    let starfield: Vec<PointOfLight> = points.iter().map(|p| predict_point_at_timestep(p, smallest_area.0)).collect();
    print_starfield(&starfield);
    0
}

#[aoc(day10, part2)]
pub fn solve_part0(points: &[PointOfLight]) -> u32 {
    let smallest_area = (0..100000).map(|ts| {
        let bounds = bounding_box_at_timestep(points, ts);
        let area = (bounds.x_max - bounds.x_min) as i64 * (bounds.y_max - bounds.y_min) as i64;
        (ts, area)
    }).min_by_key(|x| x.1).expect("couldn't find the minimum area");
    smallest_area.0
}
