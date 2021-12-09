use std::collections::HashMap;
use utils::AOCChallenge;

pub fn parse_file_segments(filename: &'static str) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let mut sequences : Vec<Vec<String>> = Vec::new();
    let mut out_segs : Vec<Vec<String>> = Vec::new();
    if let Ok(lines) = utils::read_lines(filename) {
        for line in lines {
            if let Ok(entry_str) = line {
                let entry_strs : Vec<&str> = entry_str.split("|").collect();
                let entry_seqs = entry_strs[0].split_whitespace();
                let mut seq_chars: Vec<Vec<char>> = entry_seqs.map(|x| x.chars().collect::<Vec<char>>()).collect();
                for elements in seq_chars.iter_mut() {
                    elements.sort();
                }

                let mut seq: Vec<String> = Vec::new();
                for ent in seq_chars {
                    seq.push(ent.iter().cloned().collect::<String>());
                }
                sequences.push(seq);

                let entry_segs = entry_strs[1].split_whitespace();
                let mut out_chars: Vec<Vec<char>> = entry_segs.map(|x| x.chars().collect::<Vec<char>>()).collect();
                for elements in out_chars.iter_mut() {
                    elements.sort();
                }

                let mut out_seg: Vec<String> = Vec::new();
                for ent in out_chars {
                    out_seg.push(ent.iter().cloned().collect::<String>());
                }
                out_segs.push(out_seg);
            }
        }
    }
    return (sequences, out_segs);
}

struct Day (u32);

fn unique_count(output_segs: Vec<Vec<String>>) -> u32 {
    let mut count : u32 = 0;
    for segs in output_segs {
        for seg in segs {
            let len = seg.len();
            if len == 2 || len == 3 || len == 4 || len == 7 {
                count += 1
            }
        }
    }
    return count;
}

fn decode_sequences(num_seqs: Vec<Vec<String>>) -> Vec<HashMap<String, u64>> {
    let mut decodes : Vec<HashMap<String, u64>> = Vec::new();
    for sequences in num_seqs {
        let mut map: HashMap<String, u64> = HashMap::new();
        let mut two_three_five: Vec<String> = Vec::new();
        let mut zero_six_nine: Vec<String> = Vec::new();
        let mut one : String = String::new();
        for seq in sequences {
            match seq.len() {
                2 => {
                    one.push_str(&seq);
                    map.insert(seq, 1);
                }
                3 => {
                    map.insert(seq, 7);
                }
                4 => {
                    map.insert(seq, 4);
                }
                5 => {
                    // 2, 3, or 5
                    two_three_five.push(seq);
                }
                6 => {
                    zero_six_nine.push(seq);
                }
                7 => {
                    map.insert(seq, 8);
                }
                _ => {}
            }
        }
        let mut tr_seg = 'a';
        let mut zero_nine: Vec<String> = Vec::new();
        for seq in zero_six_nine.iter() {
            if &seq.find(one.get(..1).unwrap()) == &None || &seq.find(one.get(1..2).unwrap()) == &None {
                for seg in one.chars() {
                    if seq.find(seg.to_string().as_str()) == None {
                        tr_seg = seg;
                    }
                }
                map.insert(seq.to_string(), 6);
            } else {
                zero_nine.push(seq.to_string());
            }
        }
        let mut three = String::new();
        for seq in two_three_five {
            if seq.find(one.get(..1).unwrap()) != None && seq.find(one.get(1..2).unwrap()) != None {
                three.push_str(&seq);
                map.insert(seq, 3);
            } else if seq.find(tr_seg) == None {
                map.insert(seq, 5);
            } else {
                map.insert(seq, 2);
            }
        }

        for seq in zero_nine {
            let mut found_zero = false;
            for i in 0..three.len() {
                if seq.find(three.get(i..i+1).unwrap()) == None {
                    map.insert(seq.to_string(), 0);
                    found_zero = true;
                    break;
                }
            }
            if !found_zero {
                map.insert(seq.to_string(), 9);
            }
        }
        decodes.push(map);

    }
    return decodes;
}

fn get_decoded_sum(decodes: Vec<HashMap<String, u64>>, output_segs: Vec<Vec<String>>) -> u64 {
    let mut sum: u64 = 0;
    for (i, segs) in output_segs.iter().enumerate() {
        let mut seg_val: u64 = 0;
        for seg in segs.iter() {
            seg_val = 10 * seg_val + decodes[i].get(seg).unwrap();
        }
        // println!("{}", seg_val);
        sum += seg_val;
    }
    return sum;
}

impl utils::AOCChallenge for Day {
    fn part1_impl(in_file: &'static str) {
        let (_, output_segs) = parse_file_segments(in_file);
        println!("Unique segment count: {}", unique_count(output_segs));
    }

    fn part2_impl(in_file: &'static str) {
        let (num_seqs, output_segs) = parse_file_segments(in_file);
        let codes = decode_sequences(num_seqs);
        println!("Code sum: {}", get_decoded_sum(codes, output_segs));

    }
}

fn main() {
    Day::run_part();
}