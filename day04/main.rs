use utils::AOCChallenge;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct BingoBoard {
    card : [[u32; 5]; 5],
    marked : [[bool; 5]; 5]
}

fn parse_file_bingo(filename: &'static str) -> (Vec<u32>,Vec<BingoBoard>) {
    let mut draws : Vec<u32> = Vec::new();
    let mut boards : Vec<BingoBoard> = Vec::new();
    let mut line_count = 0;
    if let Ok(lines) = utils::read_lines(filename) {
        let mut card : [[u32; 5]; 5] = [[0; 5]; 5];
        let marks : [[bool; 5]; 5] = [[false; 5]; 5];
        for line in lines {
            if let Ok(entry_str) = line {
                match line_count {
                    0 => {
                        for val in entry_str.split(",") {
                            draws.push(val.parse::<u32>().unwrap());
                        }
                    }
                    1 => {
                    }
                    _ => {
                        let card_line = (line_count - 2) % 6;
                        if card_line < 5 {
                            for (i, val) in entry_str.split_whitespace().enumerate() {
                                card[card_line][i] = val.parse::<u32>().unwrap();
                            }
                        }
                        if card_line == 4 {
                            let board = BingoBoard{ card: card, marked: marks};
                            boards.push(board);
                        }
                    }
                }
                line_count += 1;
            }
        }
    }
    return (draws, boards);
}

impl BingoBoard {

    fn check_win(&self) -> (bool, u32) {
        let mut col_scores : [u32; 5] = [0; 5];
        let mut unmarked_sum : u32 = 0;
        let mut won = false;
        for row in 0..5 {
            let mut row_score = 0;
            for col in 0..5 {
                if !self.marked[row][col] {
                    unmarked_sum += self.card[row][col]
                } else {
                    if !won {
                        row_score += 1;
                        col_scores[col] += 1;
                    }
                }
            }
            if row_score == 5 {
                won = true;
            }
        }
        if !won {
            for i in 0..5 {
                if col_scores[i] == 5 {
                    won = true;
                }
            }
        }
        if won {
            return (won, unmarked_sum);
        } else {
            return (false, 0);
        }
    }

    fn mark_board(&mut self, draw: u32) {
        for row in 0..5 {
            let col = self.card[row].iter().position(|&x| x == draw);
            if col != None {
                self.marked[row][col.unwrap()] = true;
                break;
            }
        }
    }
}

fn check_wins(boards: &[BingoBoard]) -> (bool, u32) {
    for board in boards.iter() {
        let (won, score) = board.check_win();
        if won {
            return (won, score);
        }
    }
    return (false, 0);
}

fn mark_boards(boards: &mut [BingoBoard], draw: u32) {
    for board in boards.iter_mut() {
        board.mark_board(draw);
    }
}

fn check_and_drop_wins(boards: &[BingoBoard]) -> (Vec<BingoBoard>, u32) {
    let mut remaining_boards : Vec<BingoBoard> = Vec::new();
    for board in boards.iter() {
        let (won, score) = board.check_win();
        if !won {
            remaining_boards.push(*board);
        } else {
            if boards.len() == 1 {
                return (remaining_boards, score);
            }
        }
    }
    return (remaining_boards, 0);
}

fn find_winning_score(in_file: &'static str) -> u32 {
    let (draws, boards) = parse_file_bingo(in_file);
    let boards_slice = boards.leak();
    let mut score : u32 = 0;
    for (i, draw) in draws.iter().enumerate() {
        mark_boards(boards_slice, *draw);
        if i >= 4 {
            let (won, unmarked_score) = check_wins(boards_slice);
            if won {
                println!("Unmarked: {}, Draw value: {}", unmarked_score, draw);
                score = unmarked_score * draw;
                break;
            }
        }
    }
    return score;
}

fn find_losing_score(in_file: &'static str) -> u32 {
    let (draws, boards) = parse_file_bingo(in_file);
    let mut boards_slice = boards.leak();
    let mut score : u32 = 0;
    for (i, draw) in draws.iter().enumerate() {
        mark_boards(boards_slice, *draw);
        if i >= 4 {
            let (boards, unmarked_score) = check_and_drop_wins(boards_slice);
            boards_slice = boards.leak();
            if boards_slice.is_empty() {
                println!("Unmarked: {}, Draw value: {}", unmarked_score, draw);
                score = unmarked_score * draw;
                break;
            }
        }
    }
    return score;
}

struct Day (u32);

impl utils::AOCChallenge for Day {
    fn part1_impl(in_file: &'static str) {
        println!("The winning score is {}", find_winning_score(in_file));
    }

    fn part2_impl(in_file: &'static str) {
        println!("The losing score is {}", find_losing_score(in_file));
    }
}

fn main() {
    Day::run_part();
}