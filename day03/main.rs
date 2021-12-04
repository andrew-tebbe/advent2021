use std::mem::size_of;
use utils::AOCChallenge;

fn get_bit_counts(diagnostics : Vec<&[u8]>) -> (Vec<u32>, Vec<u32>)
{
    let mut ones_count : Vec<u32> = vec![0; diagnostics[0].len()];
    let mut zero_count : Vec<u32> = vec![0; diagnostics[0].len()];
    for line in diagnostics.iter() {
        for i in 0..line.len() {
            if line[i] == 1 {
                ones_count[i] += 1
            } else {
                zero_count[i] += 1
            }
        }
    }
    return (zero_count, ones_count)
}

fn get_bit_fav(bit_strings : Vec<&[u8]>) -> (Vec<u8>,Vec<u8>) {
    let mut mc_bits = Vec::new();
    let mut lc_bits = Vec::new();
    let bs_slice = bit_strings.leak();
    for i in 0..bs_slice[0].len() {
        let (mc_bit, lc_bit) = get_bit_fav_idx(bs_slice.to_vec(), i);
        mc_bits.push(mc_bit);
        lc_bits.push(lc_bit);
    }
    return (mc_bits, lc_bits);
}

fn get_bit_fav_idx(diagnostics : Vec<&[u8]>, index: usize) -> (u8,u8) {
    let mut zero_count = 0;
    let mut ones_count = 0;
    for i in 0..diagnostics.len() {
        if diagnostics[i][index] == 0 {
            zero_count += 1;
        } else if diagnostics[i][index] == 1 {
            ones_count += 1;
        }
    }
    let mut mc_bit = 1;
    let mut lc_bit = 0;
    if ones_count > zero_count{
        mc_bit = 1;
        lc_bit = 0;
    } else if zero_count > ones_count {
        mc_bit = 0;
        lc_bit = 1;
    }
    return (mc_bit, lc_bit);
}

fn convert_bitstring_to_u32(bs: Vec<u8>) -> u32 {
    let mut val : u32 = 0;
    for i in 0..bs.len() {
        val += (bs[i] as u32) << (bs.len() - i  - 1);
    }
    return val;
}

fn get_bit_criteria_matches(bit_strings: Vec<&[u8]>, index: usize, mc: bool) -> Vec<&[u8]> {
    let bs_slice = bit_strings.leak();
    if bs_slice.len() == 1 || index >= bs_slice[0].len() {
        return bs_slice.to_vec();
    }

    let (mcb, lcb) = get_bit_fav_idx(bs_slice.to_vec(), index);
    let mut matches = Vec::new();
    for bs in bs_slice.iter() {
        if mc {
            if bs[index] == mcb {
                matches.push(*bs)
            }
        } else {
            if bs[index] == lcb {
                matches.push(*bs)
            }
        }
    }
    return get_bit_criteria_matches(matches, index + 1, mc)
}

fn calc_oxygen_co2_rates(diagnostics : Vec<&[u8]>) -> (u32, u32) {
    let diagnostics_slice = diagnostics.leak();
    let oxygen_rates = get_bit_criteria_matches(diagnostics_slice.to_vec(), 0, true);
    let co2_rates = get_bit_criteria_matches(diagnostics_slice.to_vec(), 0, false);
    // for i in diagnostics.len():
    let oxygen_rate = convert_bitstring_to_u32(oxygen_rates[0].to_vec());
    let co2_rate = convert_bitstring_to_u32(co2_rates[0].to_vec());
    println!("oxygen: {}, co2: {}", oxygen_rate, co2_rate);
    return (oxygen_rate, co2_rate);

}

fn calc_gamma_epsilon_rates(diagnostics : Vec<&[u8]>) -> (u32, u32) {
    let (mcb, _) = get_bit_fav(diagnostics);
    let shift_val = size_of::<u32>() * 8 -  mcb.len();
    let gamma_rate = convert_bitstring_to_u32(mcb);
    let epsilon_rate: u32 = ((!gamma_rate) << shift_val) >> shift_val;

    println!("Gamma: {}, Epsilon: {}", gamma_rate, epsilon_rate);
    return (gamma_rate, epsilon_rate);
}

fn get_power_consumption(diagnostics : Vec<&[u8]>) -> u32 {
    let (gamma_rate, epsilon_rate) = calc_gamma_epsilon_rates(diagnostics);
    return gamma_rate * epsilon_rate;
}

fn get_life_support(diagnostics : Vec<&[u8]>) -> u32 {
    let (oxygen_rate, co2_rate) = calc_oxygen_co2_rates(diagnostics);
    return oxygen_rate * co2_rate;
}

struct Day (u32);

impl utils::AOCChallenge for Day {
    fn part1_impl(in_file: &'static str) {
        let diagnostics = utils::parse_file_bitstring(in_file);
        println!("power consumption: {}", get_power_consumption(diagnostics));
    }

    fn part2_impl(in_file: &'static str) {
        let diagnostics = utils::parse_file_bitstring(in_file);
        println!("life support: {}", get_life_support(diagnostics));
    }
}

fn main() {
    Day::run_part();
}