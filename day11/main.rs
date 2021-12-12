use utils::AOCChallenge;

struct Day (u32);

trait Octopus {
    fn new(energy: u32) -> Self;
    fn up_energy(&mut self);
    fn flash(&mut self) -> bool;
    fn power_up(&mut self) -> bool {
        self.up_energy();
        return self.flash();
    }
    fn end_cycle(&mut self);
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct FlashingOctopus {
    flashed: bool,
    energy: u32,
}

impl Octopus for FlashingOctopus {
    fn new(energy: u32) -> FlashingOctopus {
        return FlashingOctopus{flashed: false, energy: energy};
    }

    fn up_energy(&mut self) {
        self.energy += 1;
    }

    fn flash(&mut self) -> bool {
        if !self.flashed && self.energy > 9 {
            self.flashed = true;
            return true;
        } else {
            return false;
        }
    }

    fn end_cycle(&mut self) {
        if self.flashed {
            self.energy = 0;
        }
        self.flashed = false;
    }
}

fn init_grid(filename: &'static str) -> Vec<Vec<FlashingOctopus>> {
    let mut grid: Vec<Vec<FlashingOctopus>> = Vec::new();
    let init_energies = utils::parse_file_u32_matrix(filename);
    for energy_row in init_energies {
        let mut grid_row: Vec<FlashingOctopus> = Vec::new();
        for energy in energy_row {
            grid_row.push(FlashingOctopus::new(energy));
        }
        grid.push(grid_row);
    }
    return grid;
}

fn power_adjacent(row: usize, col: usize, grid: &mut Vec<Vec<FlashingOctopus>>) {
    let row_iter: [i32; 3] = [-1, 0, 1];
    let col_iter: [i32; 3] = [-1, 0, 1];
    for i in row_iter.iter() {
        for j in col_iter.iter() {
            if *i == 0 && *j == 0 {
                continue;
            }
            if row as i32 + *i < 0 || row as i32 + *i >= grid.len() as i32 || col as i32 + *j < 0 || col as i32 + *j >= grid[0].len() as i32 {
                continue;
            }
            grid[(row as i32 + *i) as usize][(col as i32 + *j) as usize].up_energy();
        }
    }
}

fn simulate_step(grid: &mut Vec<Vec<FlashingOctopus>>) -> u32 {
    let mut flashes = 0;
    for octo_row in grid.iter_mut() {
        for octo in octo_row {
            octo.up_energy();
        }
    }

    let mut start_flashes = 1;
    while start_flashes != flashes {
        start_flashes = flashes;
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col].flash() {
                    power_adjacent(row, col, grid);
                    flashes += 1;
                }
            }
        }
    }

    for octo_row in grid.iter_mut() {
        for octo in octo_row {
            octo.end_cycle();
        }
    }
    return flashes;
}

fn simulate_steps(grid: &mut Vec<Vec<FlashingOctopus>>, steps: u32) -> u32 {
    let mut flashes = 0;
    for _ in 0..steps {
        flashes += simulate_step(grid);
    }
    return flashes;
}

fn get_flash_count(filename: &'static str, steps: u32) -> u32 {
    let mut grid = init_grid(filename);
    return simulate_steps(&mut grid, steps);
}

fn find_synched_flash(filename: &'static str) -> u32 {
    let mut grid = init_grid(filename);
    let grid_size = (grid.len() * grid[0].len()) as u32;
    let mut flashes: u32 = 0;
    let mut step = 0;
    while flashes < grid_size {
        step += 1;
        flashes = simulate_step(&mut grid);
    }
    return step;
}

impl utils::AOCChallenge for Day {
    fn part1_impl(in_file: &'static str) {
        println!("There are {} flashes", get_flash_count(in_file, 100));
    }

    fn part2_impl(in_file: &'static str) {
        println!("Flashes synch after step {}", find_synched_flash(in_file));
    }
}

fn main() {
    Day::run_part();
}