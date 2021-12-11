use utils::AOCChallenge;
use std::collections::HashMap;

struct Day (u32);

fn get_mins(heights: &Vec<Vec<u32>>) -> (Vec<usize>, Vec<usize>) {
    let mut mins_row: Vec<usize> = Vec::new();
    let mut mins_col: Vec<usize> = Vec::new();
    for row in 0..heights.len() {
        let mut last_up: bool = false;
        for col in 0..heights[row].len() {
            let up: bool = (col + 1 == heights[row].len()) || heights[row][col] < heights[row][col + 1];
            if !last_up && up {
                if (row == 0 || heights[row][col] < heights[row - 1][col]) && (row + 1 == heights.len() || heights[row][col] < heights[row + 1][col]){
                    mins_row.push(row);
                    mins_col.push(col);
                }
            }
            last_up = up;
        }
    }
    return (mins_row, mins_col);
}

fn get_risk_level(heights: Vec<Vec<u32>>) -> u32 {
    let (row_mins, col_mins) = get_mins(&heights);
    let mut risk_level: u32 = row_mins.len() as u32;
    for (i, row) in row_mins.iter().enumerate() {
        risk_level += heights[*row][col_mins[i]];
    }
    return risk_level;
}

fn propagate_id(map: &mut Vec<Vec<i32>>, row: i32, col: i32) {
    if map[row as usize][col as usize] == -1 {
        return;
    }
    let row_iter: [i32; 3] = [-1, 0, 1];
    let col_iter: [i32; 3] = [-1, 0, 1];
    for i in row_iter.iter() {
        for j in col_iter.iter() {
            if *i == 0 && *j == 0 {
                continue;
            }
            if *i == *j || (*i == -1 && *j == 1) || (*i == 1 && *j == -1){
                continue;
            }
            if row + *i < 0 || row + *i >= map.len() as i32 || col + *j < 0 || col + *j >= map[0].len() as i32 {
                continue;
            }
            if map[(row + *i) as usize][(col + *j) as usize] == -1 {
                continue;
            }
            if map[(row + *i) as usize][(col + *j) as usize] == map[row as usize][col as usize] {
                continue;
            } else {
                map[(row + *i) as usize][(col + *j) as usize] = map[row as usize][col as usize];
                propagate_id(map, row + *i, col + *j);
            }
        }
    }
}

fn get_basin_map(heights: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut map : Vec<Vec<i32>> = Vec::new();
    let mut id : i32 = 0;
    for row in 0..heights.len() {
        let mut map_row : Vec<i32> = Vec::new();
        for col in 0..heights[row].len(){
            if heights[row][col] != 9 {
                map_row.push(id);
            } else {
                map_row.push(-1);
                id += 1;
            }
        }
        map.push(map_row);
    }

    for row in 0..heights.len() {
        for col in 0..heights[row].len() {
            propagate_id(&mut map, row as i32, col as i32);
        }
    }

    let mut basin_sizes : HashMap<i32, u32> = HashMap::new();
    for row in 0..heights.len() {
        for col in 0..heights[row].len(){
            if map[row][col] != -1 {
                let sum : u32 = *basin_sizes.get(&map[row][col]).unwrap_or(&0);
                basin_sizes.insert(map[row][col], sum + 1);
            }
        }
    }
    return basin_sizes.values().map(|&x| x).collect::<Vec<_>>();

}

fn get_basin_val(heights: Vec<Vec<u32>>) -> u32 {
    let mut basins = get_basin_map(&heights);
    basins.sort();
    basins.reverse();
    return basins[0] * basins[1] * basins[2];
}


impl utils::AOCChallenge for Day {
    fn part1_impl(in_file: &'static str) {
        let heights = utils::parse_file_u32_matrix(in_file);
        println!("Risk level: {}", get_risk_level(heights));
    }

    fn part2_impl(in_file: &'static str) {
        let heights = utils::parse_file_u32_matrix(in_file);
        println!("Basin val: {}", get_basin_val(heights));
    }
}

fn main() {
    Day::run_part();
}