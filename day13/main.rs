use utils::AOCChallenge;
use std::collections::HashMap;

struct Day (u32);

#[derive(Clone)]
#[derive(Copy)]
struct Fold{axis: u8, coord: usize}

fn parse_file_coords_folds(filename: &'static str) -> (Vec<Vec<bool>>, Vec<Fold>) {
    let mut map : Vec<Vec<bool>> = Vec::new();
    let mut folds : Vec<Fold> = Vec::new();
    let mut coords_complete: bool = false;
    if let Ok(lines) = utils::read_lines(filename) {
        for line in lines {
            if let Ok(entry_str) = line {
                if entry_str == "" {
                    coords_complete = true;
                    continue;
                }
                if !coords_complete {
                    let x_coord : usize = entry_str.split(",").nth(0).unwrap().parse::<usize>().unwrap();
                    let y_coord : usize = entry_str.split(",").nth(1).unwrap().parse::<usize>().unwrap();
                    let row_len: usize;
                    if map.is_empty() {
                        row_len = 0;
                    } else {
                        row_len = map[0].len()
                    }
                    if map.len() <= y_coord {
                        let mut row: Vec<bool> = Vec::new();
                        for _ in 0..row_len {
                            row.push(false);
                        }
                        for _ in map.len()..=y_coord {
                            map.push(row.to_vec());
                        }
                    }
                    if row_len <= x_coord {
                        let mut new_cols: Vec<bool> = Vec::new();
                        for _ in row_len..=x_coord {
                            new_cols.push(false);
                        }
                        for row in map.iter_mut() {
                            let mut cols = new_cols.to_vec();
                            row.append(&mut cols);
                        }
                    }
                    map[y_coord][x_coord] = true;
                } else {
                    let axis: &str = entry_str.strip_prefix("fold along ").unwrap().split("=").nth(0).unwrap();
                    let coord: usize = entry_str.strip_prefix("fold along ").unwrap().split("=").nth(1).unwrap().parse::<usize>().unwrap();
                    let axis_id: u8;
                    if axis == "x" {
                        axis_id = 0;
                    } else {
                        axis_id = 1;
                    }
                    folds.push(Fold{axis: axis_id, coord: coord});
                }
            }
        }
    }
    return (map, folds);
}

fn fold_map(map: Vec<Vec<bool>>, folds: Vec<Fold>, num_folds: usize) -> Vec<Vec<bool>> {
    let mut cur_map: Vec<Vec<bool>> = map.to_vec();
    for i in 0..num_folds {
        let fold_coord: usize = folds[i].coord;
        if folds[i].axis == 1 {
            for row_idx in fold_coord + 1..cur_map.len() {
                for col_idx in 0..cur_map[0].len() {
                    if cur_map[row_idx][col_idx] {
                        cur_map[2 * fold_coord - row_idx][col_idx] = true;
                    }
                }
            }
            cur_map.drain(fold_coord..);
        } else {
            for col_idx in fold_coord + 1..cur_map[0].len() {
                for row_idx in 0..cur_map.len() {
                    if cur_map[row_idx][col_idx] {
                        cur_map[row_idx][2 * fold_coord - col_idx] = true;
                    }
                }
            }
            for row in cur_map.iter_mut() {
                row.drain(fold_coord..);
            }
        }
    }


    return cur_map;
}

impl utils::AOCChallenge for Day {
    fn part1_impl(in_file: &'static str) {
        let (map, folds) = parse_file_coords_folds(in_file);
        let final_map = fold_map(map, folds, 1);
        println!("Visible dots: {}", final_map.concat().iter().map(|&x| x as u32).sum::<u32>())
    }

    fn part2_impl(in_file: &'static str) {
        let code_markers: HashMap<bool, char> = HashMap::from([
            (true, '*'),
            (false, ' ')
        ]);
        let (map, folds) = parse_file_coords_folds(in_file);
        let final_map = fold_map(map, folds.to_vec(), folds.len());
        println!("Code: ");

        for row in final_map {
            let row_str = row.iter().map(|x| *code_markers.get(x).unwrap()).collect::<String>();
            println!("{:?}", row_str);
        }
    }
}

fn main() {
    Day::run_part();
}