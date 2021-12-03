use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    time::Instant,
};

fn main() {
    let mut now = Instant::now();
    let p1 = problem_one();
    println!(
        "Problem 1: {}, completed in {} us.",
        p1,
        now.elapsed().as_micros()
    );

    now = Instant::now();
    let p2 = problem_two();
    println!(
        "Problem 2: {}, completed in {} us.",
        p2,
        now.elapsed().as_micros()
    );
}

fn problem_one() -> u32 {
    let mut horizontal: u32 = 0;
    let mut vertical: u32 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let tokens = l.split_ascii_whitespace().collect::<Vec<&str>>();
                match tokens[0] {
                    "forward" => horizontal += tokens[1].parse::<u32>().unwrap(),
                    "down" => vertical += tokens[1].parse::<u32>().unwrap(),
                    "up" => vertical -= tokens[1].parse::<u32>().unwrap(),
                    _ => (),
                };
            }
        }
    }

    horizontal * vertical
}

fn problem_two() -> u32 {
    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let tokens = l.split_ascii_whitespace().collect::<Vec<&str>>();
                let amount = tokens[1].parse::<u32>().unwrap();
                match tokens[0] {
                    "forward" => {
                        horizontal += amount;
                        depth += amount * aim;
                    }
                    "down" => {
                        aim += amount;
                    }
                    "up" => {
                        aim -= amount;
                    }
                    _ => (),
                };
            }
        }
    }

    horizontal * depth
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
