use utils::AOCChallenge;

struct Day (u32);

// Get the number of fish produced by this including itself
fn simulate_days(fish: u32, days:u32, cache: &mut [i64; 256]) -> i64 {
    if fish >= days {
        return 1
    }

    let mut fish_count: i64 = 1;
    for day in (fish+1..=days).step_by(7)  {
        let new_fish : i64;
        if cache[(days - day) as usize] == -1 {
            new_fish = simulate_days(8, days-day, cache);
            cache[(days - day) as usize] = new_fish;
        } else {
            new_fish = cache[(days - day) as usize]
        }
        fish_count += new_fish;
    }
    return fish_count;
}

fn simulate_growth(fishes: Vec<u32>, days: u32) -> u64 {
    let mut resulting_fish : u64 = 0;
    let mut lookup_table : [i64; 7] = [-1; 7];
    let mut new_fish_cache : [i64; 256] = [-1; 256];
    for fish in fishes {
        let new_fish : i64;
        if lookup_table[fish as usize] == -1 {
            new_fish = simulate_days(fish, days, &mut new_fish_cache);
            lookup_table[fish as usize] = new_fish;
        } else {
            new_fish = lookup_table[fish as usize];
        }
        resulting_fish += new_fish as u64;
    }
    return resulting_fish;
}

impl utils::AOCChallenge for Day {
    fn part1_impl(in_file: &'static str) {
        let fish = utils::parse_file_u32_vec(in_file);
        println!("There are {} fish", simulate_growth(fish, 80));
    }

    fn part2_impl(in_file: &'static str) {
        let fish = utils::parse_file_u32_vec(in_file);
        println!("There are {} fish", simulate_growth(fish, 256));
    }
}

fn main() {
    Day::run_part();
}