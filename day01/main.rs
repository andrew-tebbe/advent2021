use utils::AOCChallenge;

fn get_increase_count(depths: &Vec<u32>) -> u32 {
    if depths.is_empty() {
        return 0;
    }

    let mut increases : Vec<u32> = depths.to_vec();
    increases.remove(0);

    let prev_depths : Vec<u32> = depths.to_vec();
    let mut depth_iter = prev_depths.iter();
    increases.retain(|&x| x > *depth_iter.next().unwrap());

    return increases.len() as u32;
}

fn get_increase_window_count(depths: &Vec<u32>) -> u32 {
    if depths.is_empty() {
        return 0;
    }

    let mut windows : Vec<u32> = Vec::new();

    for i in 2..depths.len() {
        windows.push(depths[i] + depths[i - 1] + depths[i - 2])
    }
    return get_increase_count(&windows);
}

struct Day (u32);

impl utils::AOCChallenge for Day {
    fn part1_impl(in_file: &'static str) {
        let depths = utils::parse_file_u32(in_file);
        println!("The depth increases {} times", get_increase_count(&depths).to_string());
    }

    fn part2_impl(in_file: &'static str) {
        let depths = utils::parse_file_u32(in_file);
        println!("The depth increases {} times", get_increase_window_count(&depths).to_string());
    }
}

fn main() {
    Day::run_part();
}