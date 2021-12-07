use std::time::Instant;

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

fn problem_one() -> i32 {
    let mut positions = get_input();
    positions.sort();
    let target_pos = positions[positions.iter().count() / 2];

    positions
        .iter()
        .fold(0, |acc, pos| acc + (pos - target_pos).abs())
}

// thanks to https://www.reddit.com/r/adventofcode/comments/rawxad/2021_day_7_part_2_i_wrote_a_paper_on_todays/
// i went to college for music so i dont know maths and algorithms
// not copying code just implementing the maths from that paper
fn problem_two() -> i32 {
    let crabs = get_input();
    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();
    let mut fuel_amounts = Vec::<i32>::new();

    for i in *min..*max {
        fuel_amounts.push(crabs.iter().fold(0, |acc, crab| {
            let distance = (crab - i).abs();
            acc + ((distance * distance) + distance) / 2
        }));
    }

    *fuel_amounts.iter().min().unwrap()
}

fn get_input() -> Vec<i32> {
    let mut res = Vec::<i32>::new();
    include_str!("../input.txt")
        .split(',')
        .for_each(|s| res.push(s.parse::<i32>().unwrap()));
    res
}
