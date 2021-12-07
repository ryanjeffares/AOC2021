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
    )
}

fn problem_one() -> u32 {
    let input = read_lines();
    input.iter().enumerate().fold(0, |acc, (index, num)| {
        if index == 0 {
            acc + 0
        } else {
            acc + if num > &input[index - 1] { 1 } else { 0 }
        }
    })
}

fn problem_two() -> u32 {
    let input = read_lines();
    input.iter().enumerate().fold(0, |acc, (index, num)| {
        acc + match index {
            0..=2 => 0,
            _ => {
                if num + input[index - 1] + input[index - 2]
                    > input[index - 1] + input[index - 2] + input[index - 3]
                {
                    1
                } else {
                    0
                }
            }
        }
    })
}

fn read_lines() -> Vec<u32> {
    include_str!("../input.txt")
        .split('\n')
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect()
}
