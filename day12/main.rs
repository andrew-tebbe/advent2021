use utils::AOCChallenge;
use std::collections::HashMap;

struct Day (u32);

#[derive(Debug)]
#[derive(Clone)]
struct Cave {
    connections: Vec<String>,
    small: bool,
    start: bool,
    end: bool
}

fn parse_caves(filename: &'static str) -> HashMap<String, Cave> {
    let mut caves: HashMap<String, Cave> = HashMap::new();
    if let Ok(lines) = utils::read_lines(filename) {
        for line in lines {
            if let Ok(entry_str) = line {
                let cave_connections: Vec<&str> = entry_str.split("-").collect();
                let con_1 = cave_connections[0];
                let con_2 = cave_connections[1];
                if caves.get(con_1).is_none() {
                    let small_cave = con_1.chars().nth(0).unwrap().is_lowercase();
                    let cave = Cave{connections: vec![con_2.to_string()], small: small_cave, start: con_1 == "start", end: con_1 == "end"};
                    caves.insert(con_1.to_string(), cave);
                } else {
                    let cave = caves.get_mut(con_1).unwrap();
                    cave.connections.push(con_2.to_string());
                }
                if caves.get(con_2).is_none() {
                    let small_cave = con_2.chars().nth(0).unwrap().is_lowercase();
                    let cave = Cave{connections: vec![con_1.to_string()], small: small_cave, start: con_2 == "start", end: con_2 == "end"};
                    caves.insert(con_2.to_string(), cave);
                } else {
                    let cave = caves.get_mut(con_2).unwrap();
                    cave.connections.push(con_1.to_string())
                }
            }
        }
    }
    return caves;
}

fn find_path(caves: &HashMap<String, Cave>, current_path: Vec<String>) -> Vec<Vec<String>> {
    let last_cave = current_path.last().unwrap();
    let mut paths: Vec<Vec<String>> = Vec::new();
    if last_cave == "end" {
        return vec![current_path];
    }
    let next_caves = &caves.get(last_cave).unwrap().connections;
    for cave in next_caves {
        if current_path.contains(cave) && caves.get(cave).unwrap().small {
            continue;
        }
        let mut new_path = current_path.to_vec();
        new_path.push(cave.to_string());
        paths.extend(find_path(caves, new_path));
    }
    return paths;
}

fn find_dup_path(caves: &HashMap<String, Cave>, current_path: Vec<String>) -> Vec<Vec<String>> {
    let last_cave = current_path.last().unwrap();
    let mut paths: Vec<Vec<String>> = Vec::new();
    if last_cave == "end" {
        return vec![current_path];
    }
    let next_caves = &caves.get(last_cave).unwrap().connections;
    for cave in next_caves {
        if current_path.contains(cave) && caves.get(cave).unwrap().small {
            if caves.get(cave).unwrap().start {
                continue;
            }
            let mut small_caves = current_path.iter().filter(|&c| caves.get(c).unwrap().small).collect::<Vec<&String>>();
            small_caves.sort();
            let mut small_caves_already_duped = false;
            for i in 1..small_caves.len() {
                if small_caves[i] == small_caves[i-1] {
                    small_caves_already_duped = true;
                    break;
                }
            }
            if small_caves_already_duped {
                continue;
            }
        }
        let mut new_path = current_path.to_vec();
        new_path.push(cave.to_string());
        paths.extend(find_dup_path(caves, new_path));
    }
    return paths;
}

fn find_paths(caves: HashMap<String, Cave>) -> Vec<Vec<String>> {
    return find_path(&caves, vec![String::from("start")]);
}

fn find_dup_paths(caves: HashMap<String, Cave>) -> Vec<Vec<String>> {
    return find_dup_path(&caves, vec![String::from("start")]);
}

impl utils::AOCChallenge for Day {
    fn part1_impl(in_file: &'static str) {
        let caves = parse_caves(in_file);
        let paths = find_paths(caves);
        println!("The number of paths is {}", paths.len());
    }

    fn part2_impl(in_file: &'static str) {
        let caves = parse_caves(in_file);
        let paths = find_dup_paths(caves);
        println!("The number of paths is {}", paths.len());
    }
}

fn main() {
    Day::run_part();
}