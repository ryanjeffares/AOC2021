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

fn problem_one() -> u32 {
    let mut epsilon_rate: u32 = 0;
    let mut gamma_rate: u32 = 0;

    let lines: Vec<&[u8]> = include_str!("../input.txt")
        .split('\n')
        .map(|l| l.trim().as_bytes())
        .collect();

    let length = lines[0].len();
    let mut one_counts = vec![0u32; length];
    let mut zero_counts = vec![0u32; length];

    for line in lines {
        for i in 0..length {
            match line[i] {
                48 => zero_counts[i] += 1,
                49 => one_counts[i] += 1,
                _ => (),
            }
        }
    }

    for i in 0..length {
        if one_counts[i] > zero_counts[i] {
            gamma_rate |= 1 << (length - 1 - i);
        } else {
            epsilon_rate |= 1 << (length - 1 - i);
        }
    }

    epsilon_rate * gamma_rate
}

fn problem_two() -> u32 {
    let lines: Vec<&[u8]> = include_str!("../input.txt")
        .split('\n')
        .map(|l| l.trim().as_bytes())
        .collect();

    let oxygen_rating = get_rating(&lines, 0, true);
    let co2_rating = get_rating(&lines, 0, false);

    oxygen_rating * co2_rating
}

fn get_rating(lines: &Vec<&[u8]>, pos_to_check: usize, oxygen: bool) -> u32 {
    if lines.len() == 1 {
        let mut result: u32 = 0;
        let length = lines[0].len();
        for i in 0..length {
            if lines[0][i] == 49 {
                result |= 1 << (length - 1 - i);
            }
        }

        return result;
    }

    let mut starts_with_one = Vec::<&[u8]>::new();
    let mut starts_with_zero = Vec::<&[u8]>::new();

    for line in lines {
        match line[pos_to_check] {
            48 => starts_with_zero.push(line),
            49 => starts_with_one.push(line),
            _ => (),
        }
    }

    if oxygen {
        if starts_with_one.len() >= starts_with_zero.len() {
            get_rating(&starts_with_one, pos_to_check + 1, true)
        } else {
            get_rating(&starts_with_zero, pos_to_check + 1, true)
        }
    } else {
        if starts_with_zero.len() > starts_with_one.len() {
            get_rating(&starts_with_one, pos_to_check + 1, false)
        } else {
            get_rating(&starts_with_zero, pos_to_check + 1, false)
        }
    }
}
