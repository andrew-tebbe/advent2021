use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_file_u32(filename: &'static str) -> Vec<u32> {
    let mut entries : Vec<u32> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(entry_str) = line {
                let entry_val : u32 = entry_str.parse::<u32>().unwrap();
                entries.push(entry_val);
            }
        }
    }
    return entries;
}

pub trait AOCChallenge {
    fn part1_impl(in_file: &'static str);

    fn part2_impl(in_file: &'static str);

    fn part1() {
        println!("Running Part 1");
        Self::part1_impl("input.txt");
    }

    fn part2() {
        println!("Running Part 2");
        Self::part2_impl("input.txt");
    }

    fn example1() {
        println!("Running Example 1");
        Self::part1_impl("example.txt");
    }

    fn example2() {
        println!("Running Example 2");
        Self::part2_impl("example.txt");
    }

    fn run_part() {
        let args: Vec<String> = env::args().collect();
        match args.len() {
            1 => {
                Self::help();
            }
            _ => {
                if let Ok(part_num) = args[1].parse::<u32>() {
                    match part_num {
                        1 => {
                            Self::part1();
                        }
                        2 => {
                            Self::part2();
                        }
                        3 => {
                            Self::example1();
                        }
                        4 => {
                            Self::example2();
                        }
                        _ => {
                            Self::help();
                        }
                    }
                }
                else {
                    Self::help();
                }
            }
        }
    }

    fn help() {
        println!("Must provide a case to run");
        println!("\t1: Part 1");
        println!("\t2: Part 2");
        println!("\t3: Example 1");
        println!("\t4: Example 2");
    }
}