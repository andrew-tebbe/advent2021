use utils::AOCChallenge;

struct Day (u32);

fn find_optimum_position(positions: Vec<u32>) -> (u32, u32) {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    let mut weights: Vec<u32> = vec![0; (max - min + 1) as usize];
    let mut right_weight: u32 = positions.len() as u32;
    let mut right_distance: u32 = 0;
    let mut left_distance: u32 = 0;
    for pos in positions.iter() {
        weights[(pos - min) as usize] += 1;
        right_distance += pos - min;
    }
    let mut left_weight: u32 = weights[0];
    right_weight -= weights[0];
    let mut min_distance: u32 = left_distance + right_distance;
    let mut min_pos: u32 = min;
    for i in 1..weights.len() {
        left_distance += left_weight;
        left_weight += weights[i];
        right_distance -= right_weight;
        right_weight -= weights[i];
        if left_distance + right_distance < min_distance {
            min_distance = left_distance + right_distance;
            min_pos = i as u32 + min;
        }
    }

    return (min_pos, min_distance);
}

fn find_optimum_weighted_position(positions: Vec<u32>) -> (u32, u32) {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    let mut weights: Vec<u32> = vec![0; (max - min + 1) as usize];
    for pos in positions.iter() {
        weights[(pos - min) as usize] += 1;
    }
    let mut min_distance: u32 = 0;
    let mut min_pos: u32 = min;
    for i in 0..weights.len() {
        let mut right_distance: u32 = 0;
        let mut left_distance: u32 = 0;
        for j in 0..i {
            let left_offset = (i - j) as u32;
            left_distance += weights[j] * left_offset * (left_offset + 1) / 2;
        }
        for j in i + 1..weights.len() {
            let right_offset = (j - i) as u32;
            right_distance += weights[j] * right_offset * (right_offset + 1) / 2;
        }
        if left_distance + right_distance < min_distance || min_distance == 0 {
            min_distance = left_distance + right_distance;
            min_pos = i as u32 + min;
        }
    }

    return (min_pos, min_distance);
}

impl utils::AOCChallenge for Day {
    fn part1_impl(in_file: &'static str) {
        let orig_pos = utils::parse_file_u32_vec(in_file);
        let (pos, distance) = find_optimum_position(orig_pos);
        println!("Target Position: {}, distance: {}", pos, distance);
    }

    fn part2_impl(in_file: &'static str) {
        let orig_pos = utils::parse_file_u32_vec(in_file);
        let (pos, distance) = find_optimum_weighted_position(orig_pos);
        println!("Target Position: {}, distance: {}", pos, distance);
    }
}

fn main() {
    Day::run_part();
}