use utils::AOCChallenge;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

struct Day (u32);


fn parse_chunk_lines(filename: &'static str) -> Vec<String> {
    let mut entries : Vec<String> = Vec::new();
    if let Ok(lines) = utils::read_lines(filename) {
        for line in lines {
            if let Ok(entry_str) = line {
                entries.push(entry_str.to_string());
            }
        }
    }
    return entries;
}

lazy_static! {
    static ref CLOSE_CHAR: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>')
    ]);
}

lazy_static! {
    static ref ERROR_SCORE: HashMap<char, u32> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137)
    ]);
}

lazy_static! {
    static ref COMPLETE_SCORE: HashMap<char, u64> = HashMap::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4)
    ]);
}


fn calc_syntax_error_score(syntax_errors: Vec<char>) -> u32 {
    let mut score = 0;
    for syntax_error in syntax_errors {
        score += ERROR_SCORE.get(&syntax_error).unwrap();
    }
    return score;
}

fn calc_autocompletion_score(autocompletes: Vec<Vec<char>>) -> u64 {
    let mut line_scores: Vec<u64> = Vec::new();
    for autocomplete in autocompletes {
        let mut line_score: u64 = 0;
        for cur_char in autocomplete {
            line_score = line_score * 5 + COMPLETE_SCORE.get(&cur_char).unwrap();
        }
        line_scores.push(line_score);
    }
    line_scores.sort();
    let mid_idx = line_scores.len() / 2;
    return line_scores[mid_idx];
}

fn get_autocompletion_char(incompletes: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut autocompletes: Vec<Vec<char>> = Vec::new();
    for incomplete in incompletes {
        let mut line_autocomplete: Vec<char> = Vec::new();
        for cur_char in incomplete.iter().rev() {
            line_autocomplete.push(*CLOSE_CHAR.get(&cur_char).unwrap());
        }
        autocompletes.push(line_autocomplete);
    }
    return autocompletes;
}

fn get_syntax_errors(chunks: Vec<String>) -> (Vec<char>, Vec<Vec<char>>) {
    let mut syntax_errors : Vec<char> = Vec::new();
    let mut unclosed_chunks : Vec<Vec<char>> = Vec::new();
    for chunk in chunks {
        let mut syntax_err: bool = false;
        let mut open_chunks : Vec<char> = Vec::new();
        for chunk_char in chunk.chars() {
            if CLOSE_CHAR.contains_key(&chunk_char) {
                open_chunks.push(chunk_char);
                continue;
            }
            let cur_char = open_chunks.pop();
            if cur_char != None && chunk_char != *CLOSE_CHAR.get(&cur_char.unwrap()).unwrap() {
                syntax_errors.push(chunk_char);
                syntax_err = true;
                break;
            }
        }
        if !syntax_err {
            unclosed_chunks.push(open_chunks);
        }
    }
    return (syntax_errors, unclosed_chunks)
}

fn get_syntax_error_score(chunks: Vec<String>) -> u32 {
    let (syntax_errors, _) = get_syntax_errors(chunks);
    return calc_syntax_error_score(syntax_errors);
}

fn get_autocompletion_score(chunks: Vec<String>) -> u64 {
    let (_, unclosed_chunks) = get_syntax_errors(chunks);
    let autocompletes = get_autocompletion_char(unclosed_chunks);
    return calc_autocompletion_score(autocompletes);
}

impl utils::AOCChallenge for Day {
    fn part1_impl(in_file: &'static str) {
        let chunks = parse_chunk_lines(in_file);
        println!("Syntax error score: {}", get_syntax_error_score(chunks));
    }

    fn part2_impl(in_file: &'static str) {
        let chunks = parse_chunk_lines(in_file);
        println!("Autocomplete score: {}", get_autocompletion_score(chunks));
    }
}

fn main() {
    Day::run_part();
}