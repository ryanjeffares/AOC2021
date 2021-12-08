use std::time::Instant;

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

    include_str!("../input.txt").split('\n').for_each(|line| {
        let mut split = line.trim().split(' ');
        match split.nth(0).unwrap() {
            "forward" => horizontal += split.nth(0).unwrap().parse::<u32>().unwrap(),
            "down" => vertical += split.nth(0).unwrap().parse::<u32>().unwrap(),
            "up" => vertical -= split.nth(0).unwrap().parse::<u32>().unwrap(),
            _ => (),
        }
    });

    horizontal * vertical
}

fn problem_two() -> u32 {
    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;

    include_str!("../input.txt").split('\n').for_each(|line| {
        let mut split = line.trim().split(' ');
        match split.nth(0).unwrap() {
            "forward" => {
                let amount = split.nth(0).unwrap().parse::<u32>().unwrap();
                horizontal += amount;
                depth += amount * aim;
            }
            "down" => {
                aim += split.nth(0).unwrap().parse::<u32>().unwrap();
            }
            "up" => {
                aim -= split.nth(0).unwrap().parse::<u32>().unwrap();
            }
            _ => (),
        }
    });

    horizontal * depth
}
