use std::{
    fs::File,
    io::{self, BufRead, Error},
    path::Path, time::Instant,
};

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

    if let Ok(lines) = read_lines("./input.txt") {
        let length = lines[0].len();
        let mut one_counts = vec![0u32; length];
        let mut zero_counts = vec![0u32; length];

        for line in lines {
            for i in 0..length {
                match line[i] as char {
                    '0' => zero_counts[i] += 1,
                    '1' => one_counts[i] += 1,
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
    }

    epsilon_rate * gamma_rate    
}

fn problem_two() -> u32 {
    let mut oxygen_rating: u32 =0;
    let mut co2_rating:u32 = 0;
    
    if let Ok(lines) = read_lines("./input.txt") {
        oxygen_rating = get_rating(&lines, 0, true);
        co2_rating = get_rating(&lines, 0, false);
    }

    oxygen_rating * co2_rating
}

fn get_rating(lines: &Vec<Vec<u8>>, pos_to_check: usize, oxygen: bool) -> u32 {    
    if lines.len() == 1 {
        let mut result: u32 = 0;
        let length = lines[0].len();
        for i in 0..length {
            if lines[0][i] as char == '1' {
                result |= 1 << (length - 1 - i);
            }
        }

        return result
    }

    let mut starts_with_one = Vec::<Vec<u8>>::new();
    let mut starts_with_zero = Vec::<Vec<u8>>::new();

    for line in lines {
        match line[pos_to_check] as char {
            '0' => {
                starts_with_zero.push(line.to_vec())
            },
            '1' => {
                starts_with_one.push(line.to_vec())
            },
            _ => ()
        }
    }

    if oxygen {
        if starts_with_one.len() >= starts_with_zero.len() {
            get_rating(&starts_with_one, pos_to_check + 1, true)
        } else {
            get_rating(&starts_with_zero, pos_to_check + 1, true)
        }    
    }
    else {
        if starts_with_zero.len() > starts_with_one.len() {
            get_rating(&starts_with_one, pos_to_check + 1, false)
        } else {
            get_rating(&starts_with_zero, pos_to_check + 1, false)
        }  
    }
}

fn read_lines<P>(filename: P) -> Result<Vec<Vec<u8>>, Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut res = Vec::<Vec<u8>>::new();
    for line in lines {
        if let Ok(l) = line {
            res.push(l.as_bytes().to_vec());
        }
    }

    Ok(res)
}
