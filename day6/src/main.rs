use std::time::Instant;

pub fn main() {
    let mut now = Instant::now();
    let p1 = solve(80);
    println!(
        "Problem 1: {}, Completed in {} us.", 
        p1,
        now.elapsed().as_micros()
    );

    now = Instant::now();
    let p2 = solve(256);
    println!(
        "Problem 2: {}, Completed in {} us.", 
        p2,
        now.elapsed().as_micros()
    );
}

fn solve(days: usize) -> usize {
    let mut counts = include_str!("../input.txt")
        .split(',')
        .fold([0; 9], |mut arr, num| {
            arr[num.parse::<usize>().unwrap()] += 1;
            arr
    });

    for day in 0..days {
        counts[(day + 7) % 9] += counts[day % 9];
    }

    counts.iter().sum()
}
