use std::{fs, time::Instant};

const BOARD_WIDTH: usize = 5;

fn main() {
    let mut now = Instant::now();
    let p1 = problem_one();
    println!(
        "Problem 1: {}, Completed in {} us.",
        p1,
        now.elapsed().as_micros()
    );

    now = Instant::now();
    let p2 = problem_two();
    println!(
        "Problem 2: {}, Completed in {} us.",
        p2,
        now.elapsed().as_micros()
    );
}

fn problem_one() -> u32 {
    let (input_numbers, mut bingo_boards) = get_input();

    for input_number in input_numbers {
        for board in bingo_boards.iter_mut() {
            if board.check_success(input_number) {
                return board.get_score(input_number as u32);
            }
        }
    }

    0
}

fn problem_two() -> u32 {
    let (input_numbers, mut bingo_boards) = get_input();

    for input_number in input_numbers {
        if bingo_boards.iter().filter(|b| !b.has_won).count() == 1 {
            if let Some(index) = bingo_boards.iter().position(|b| !b.has_won) {
                if bingo_boards[index].check_success(input_number) {
                    return bingo_boards[index].get_score(input_number as u32);
                }
            }
        }

        for board in bingo_boards.iter_mut() {
            board.check_success(input_number);
        }
    }

    0
}

fn get_input() -> (Vec<u8>, Vec<BingoBoard>) {
    let input_numbers_str =
        fs::read_to_string("./input_numbers.txt").expect("Couldn't read input_numers.txt");
    let mut input_numbers = Vec::<u8>::new();

    for num in input_numbers_str.split(',') {
        input_numbers.push(num.parse::<u8>().unwrap());
    }

    let mut bingo_boards = Vec::<BingoBoard>::new();
    let input_boards_str = fs::read_to_string("./input.txt").expect("Couldn't read input.txt");

    let mut matrix = [[0u8; BOARD_WIDTH]; BOARD_WIDTH];
    let mut counter = 0;

    let input_lines = input_boards_str.split('\n');
    for input_line in input_lines {
        if input_line.trim().is_empty() {
            continue;
        }

        let nums = input_line.trim().split(' ');
        let mut num_index = 0;
        for num in nums {
            let trimmed = num.trim();
            if !trimmed.is_empty() {
                matrix[counter][num_index] = num.parse::<u8>().unwrap();
                num_index += 1;
            }
        }

        counter += 1;

        if counter == 5 {
            bingo_boards.push(BingoBoard::new(&matrix));
            counter = 0;
        }
    }

    (input_numbers, bingo_boards)
}

struct BingoNumber {
    pub number: u8,
    pub found: bool,
}

impl BingoNumber {
    pub fn new(number: u8) -> Self {
        BingoNumber {
            number,
            found: false,
        }
    }
}

struct BingoBoard {
    rows: Vec<Vec<BingoNumber>>,
    pub has_won: bool,
}

impl BingoBoard {
    pub fn new(input: &[[u8; BOARD_WIDTH]; BOARD_WIDTH]) -> Self {
        let mut numbers = Vec::<Vec<BingoNumber>>::new();

        for y in 0..5 {
            numbers.push(Vec::<BingoNumber>::new());
            for x in 0..5 {
                numbers[y].push(BingoNumber::new(input[y][x]));
            }
        }

        BingoBoard { 
            rows: numbers,
            has_won: false,
        }
    }

    pub fn check_success(&mut self, number: u8) -> bool {
        for row in 0..5 {
            if let Some(index) = self.rows[row].iter().position(|b| b.number == number) {
                self.rows[row][index].found = true;

                // check that row
                if self.rows[row].iter().all(|b| b.found) {
                    self.has_won = true;
                    return true;
                }

                // check that column
                let mut found = true;
                for y in 0..5 {
                    if !self.rows[y][index].found {
                        found = false;
                    }
                }

                if found {
                    self.has_won = true;
                }

                return found;
            }
        }

        false
    }

    pub fn get_score(&self, winning_number: u32) -> u32 {
        let mut sum = 0u32;
        for row in &self.rows {
            for num in row.iter().filter(|b| !b.found) {
                sum += num.number as u32;
            }
        }

        sum * winning_number
    }
}
