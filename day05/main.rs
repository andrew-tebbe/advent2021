use std::cmp::max;
use std::cmp::min;
use utils::AOCChallenge;

#[derive(Debug)]
struct Line {
    x1 : i32,
    y1 : i32,
    x2 : i32,
    y2 : i32
}

fn parse_vent_lines(filename: &'static str) -> Vec<Line> {
    let mut vents : Vec<Line> = Vec::new();
    if let Ok(lines) = utils::read_lines(filename) {
        for line in lines {
            if let Ok(entry_str) = line {
                let points : Vec<&str> = entry_str.split(" -> ").collect();
                let coords1 : Vec<&str> = points[0].split(",").collect();
                let coords2 : Vec<&str> = points[1].split(",").collect();
                let vent = Line{
                    x1: coords1[0].parse::<i32>().unwrap(),
                    y1: coords1[1].parse::<i32>().unwrap(),
                    x2: coords2[0].parse::<i32>().unwrap(),
                    y2: coords2[1].parse::<i32>().unwrap()};
                vents.push(vent);
            }
        }
    }
    return vents;
}

fn create_map(vents: Vec<Line>, ignore_sloped: bool) -> Vec<Vec<u32>> {
    let mut vent_map: Vec<Vec<u32>> = Vec::new();
    for vent in vents.iter() {
        if max(vent.y1, vent.y2) as usize >= vent_map.len() {
            for _ in 0..=(max(vent.y1, vent.y2) as usize - vent_map.len()) {
                vent_map.push(Vec::new());
            }
        }
        let vert = vent.x1 == vent.x2;
        let mut slope : i32 = 0;
        if !vert {
            slope = (vent.y2 - vent.y1) / (vent.x2 - vent.x1);
        }

        if ignore_sloped && slope != 0 {
            continue
        }

        let x_range;
        if vent.x2 >= vent.x1 {
            x_range = vent.x1..=vent.x2;
        } else {
            x_range = vent.x2..=vent.x1;
        }
        for x_coord in x_range {
            if vert {
                for y_coord in min(vent.y1, vent.y2)..=max(vent.y1, vent.y2) {
                    if x_coord as usize >= vent_map[y_coord as usize].len() {
                        for _ in 0..=(x_coord as usize - vent_map[y_coord as usize].len()) {
                            vent_map[y_coord as usize].push(0);
                        }
                    }
                    vent_map[y_coord as usize][x_coord as usize] += 1;
                }
                break;
            }
            else {
                let y_coord = ((x_coord - vent.x1) * slope + vent.y1) as usize;
                if x_coord as usize >= vent_map[y_coord].len() {
                    for _ in 0..=(x_coord as usize - vent_map[y_coord].len()) {
                        vent_map[y_coord].push(0);
                    }
                }
                vent_map[y_coord][x_coord as usize] += 1;
            }
        }
    }
    return vent_map;
}

fn get_intersect_count(filename: &'static str, ignore_sloped: bool) -> u32 {
    let vents = parse_vent_lines(filename);
    let map = create_map(vents, ignore_sloped);
    // println!("{:?}", map);
    let mut intersections = 0;
    for row in map.iter() {
        for pos in row.iter() {
            if *pos > 1 {
                intersections += 1;
            }
        }
    }
    return intersections;
}

struct Day (u32);

impl utils::AOCChallenge for Day {
    fn part1_impl(in_file: &'static str) {
        println!("Intersection count: {}", get_intersect_count(in_file, true));
    }

    fn part2_impl(in_file: &'static str) {
        println!("Intersection count: {}", get_intersect_count(in_file, false));
    }
}

fn main() {
    Day::run_part();
}