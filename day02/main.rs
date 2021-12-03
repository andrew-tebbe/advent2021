use utils::AOCChallenge;


fn get_position_product(movements: &Vec<utils::StrU32Pair>) -> i64 {
    let mut depth : i64 = 0;
    let mut horz : i64 = 0;
    for movement in movements.iter() {
        match movement.string.as_str() {
            "forward" => {
                horz += movement.int as i64;
            }
            "up" => {
                depth -= movement.int as i64;
            }
            "down" => {
                depth += movement.int as i64;
            }
            _ => {}
        }
    }
    return depth * horz;
}

fn get_position_product_with_aim(movements: &Vec<utils::StrU32Pair>) -> i64 {
    let mut depth : i64 = 0;
    let mut horz : i64 = 0;
    let mut aim : i64 = 0;
    for movement in movements.iter() {
        match movement.string.as_str() {
            "forward" => {
                horz += movement.int as i64;
                depth += aim * movement.int as i64
            }
            "up" => {
                aim -= movement.int as i64;
            }
            "down" => {
                aim += movement.int as i64;
            }
            _ => {}
        }
    }
    return depth * horz;
}

struct Day (u32);

impl utils::AOCChallenge for Day {
    fn part1_impl(in_file: &'static str) {
        let movements = utils::parse_file_str_u32(in_file);
        println!("The final product is {}", get_position_product(&movements).to_string());
    }

    fn part2_impl(in_file: &'static str) {
        let movements = utils::parse_file_str_u32(in_file);
        println!("The final product is {}", get_position_product_with_aim(&movements).to_string());
    }
}

fn main() {
    Day::run_part();
}