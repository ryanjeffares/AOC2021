use std::fs::File;
use std::u32::MAX;
use std::io::{self, BufRead, Error};
use std::path::Path;
use std::time::Instant;
use std::vec;

fn main() {
    
    let mut now = Instant::now();
    let p1 = problem_one();
    println!(
        "Problem 1: {}, completed in {} us.", 
        p1, now.elapsed().as_micros()
    );
    
    now = Instant::now();
    let p2 = problem_two();
    println!(
        "Problem 2: {}, completed in {} us.",
        p2,
        now.elapsed().as_micros()
    )
}

fn problem_one() -> u32 {
    let mut previous: u32 = MAX;
    let mut num_increases: u32 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {                
            if line > previous {
                num_increases += 1;
            }
            previous = line;
        }
    }

    num_increases
}

fn problem_two() -> u32 {

    let mut sums: Vec<u32> = vec![];

    if let Ok(lines) = read_lines("./input.txt") {
        for i in 2..lines.len() {
            sums.push(lines[i] + lines[i - 1] + lines[i - 2]);
        }
    }

    let mut num_increases: u32 = 0;

    for i in 1..sums.len() {
        if sums[i] > sums[i - 1] {
            num_increases += 1;
        }
    }

    num_increases
}

fn read_lines<P>(filename: P) -> Result<Vec<u32>, Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut nums: Vec<u32> = vec![];
    for line in lines {
        if let Ok(l) = line {
            nums.push(l.parse::<u32>().unwrap());
        }
    }

    Ok(nums)
}
