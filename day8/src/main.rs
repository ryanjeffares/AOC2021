use std::{collections::HashMap, time::Instant};

type Position = u8;

const TOP         : Position = 1;
const TOP_LEFT    : Position = 1 << 1;
const TOP_RIGHT   : Position = 1 << 2;
const MIDDLE      : Position = 1 << 3;
const BOTTOM_LEFT : Position = 1 << 4;
const BOTTOM_RIGHT: Position = 1 << 5;
const BOTTOM      : Position = 1 << 6;

const ZERO : Position = TOP | TOP_LEFT | TOP_RIGHT | BOTTOM_LEFT | BOTTOM_RIGHT | BOTTOM;
const ONE  : Position = TOP_RIGHT | BOTTOM_RIGHT;
const TWO  : Position = TOP | TOP_RIGHT | MIDDLE | BOTTOM_LEFT | BOTTOM;
const THREE: Position = TOP | TOP_RIGHT | MIDDLE | BOTTOM_RIGHT | BOTTOM;
const FOUR : Position = TOP_LEFT | TOP_RIGHT | MIDDLE | BOTTOM_RIGHT;
const FIVE : Position = TOP | TOP_LEFT | MIDDLE | BOTTOM_RIGHT | BOTTOM;
const SIX  : Position = TOP | TOP_LEFT | MIDDLE | BOTTOM_LEFT | BOTTOM_RIGHT | BOTTOM;
const SEVEN: Position = TOP | TOP_RIGHT | BOTTOM_RIGHT;
const EIGHT: Position = TOP | TOP_LEFT | TOP_RIGHT | MIDDLE | BOTTOM_LEFT | BOTTOM_RIGHT | BOTTOM;
const NINE : Position = TOP | TOP_LEFT | TOP_RIGHT | MIDDLE | BOTTOM_RIGHT | BOTTOM;

                                // 1  4  7  8
const UNIQUE_COUNTS: [usize; 4] = [2, 4, 3, 7];
const NUMBER_REPS  : [Position; 10] = [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

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
    include_str!("../input.txt")
        .split('\n')
        .fold(0, |acc, line| {
            let parts = line.trim().split('|');
            acc + parts
                .last()
                .unwrap()
                .trim()
                .split(' ')
                .filter(|s| UNIQUE_COUNTS.contains(&s.len()))
                .count() as u32
        })
}

fn problem_two() -> u32 {
    include_str!("../input.txt")
        .split('\n')
        .fold(0, |acc, line| {
            let mut split = line.split('|');
            acc + decode(split.nth(0).unwrap(), split.nth(0).unwrap())
        })
}

fn decode(input: &str, output: &str) -> u32 {
    let mut positions = HashMap::<char, Position>::new();
    let input_nums: Vec<&str> = input.trim().split(' ').collect();
    
    // known numbers
    let one = input_nums.iter().find(|s| s.len() == 2).unwrap();
    let four = input_nums.iter().find(|s| s.len() == 4).unwrap();
    let seven = input_nums.iter().find(|s| s.len() == 3).unwrap();
    let eight = input_nums.iter().find(|s| s.len() == 7).unwrap();

    let four_bytes = four.as_bytes();
    let seven_bytes = seven.as_bytes();
    let eight_bytes = eight.as_bytes();

    // can get top segment by comparing 1 and 7
    positions.insert(
        seven_bytes[seven.find(|c| !one.contains(c)).unwrap()] as char,
        TOP,
    );

    // can get bottom left by comparing 0/9, 4, and 8
    // only segment thats not in 4, is in 8, and in 1 of the two six segment numbers
    positions.insert(
        eight_bytes[eight
            .find(|c| {
                !four.contains(c)
                    && input_nums
                        .iter()
                        .filter(|s| s.len() == 6 && s.contains(c))
                        .count()
                        == 2
            })
            .unwrap()] as char,
        BOTTOM_LEFT,
    );

    // can now get bottom - only segment that isnt in 4 or 7 but is in 8, and isnt the bottom left
    positions.insert(
        eight_bytes[eight
            .find(|c| !four.contains(c) && !seven.contains(c) && !positions.contains_key(&c))
            .unwrap()] as char,
        BOTTOM,
    );

    // can get top right - only segment that is in 4 and 1, and 2 of the six segment numbers
    positions.insert(
        four_bytes[four
            .find(|c| {
                one.contains(c)
                    && input_nums
                        .iter()
                        .filter(|s| s.len() == 6 && s.contains(c))
                        .count()
                        == 2
            })
            .unwrap()] as char,
        TOP_RIGHT,
    );

    // can get middle - only segment that is in 4, not already in map, and present in 2 of the six segment numbers
    positions.insert(
        four_bytes[four
            .find(|c| {
                input_nums
                    .iter()
                    .filter(|s| s.len() == 6 && s.contains(c))
                    .count()
                    == 2
                    && !positions.contains_key(&c)
            })
            .unwrap()] as char,
        MIDDLE,
    );

    // can get bottom right - only segment thats in 7, in 4, not top right
    positions.insert(
        seven_bytes[seven
            .find(|c| four.contains(c) && !positions.contains_key(&c))
            .unwrap()] as char,
        BOTTOM_RIGHT,
    );

    // have top, bottom left, bottom, top right, middle, bottom right
    // top left - only segment thats in 4, not in 1, not middle
    positions.insert(
        four_bytes[four
            .find(|c| !one.contains(c) && !positions.contains_key(&c))
            .unwrap()] as char,
        TOP_LEFT,
    );

    output.trim()
        .split(' ')
        .enumerate()
        .fold(0, |sum, (index, s)| {
            sum + (NUMBER_REPS
                .iter()
                .position(|&x| x == s.chars().fold(0, |segment, c| segment | positions[&c]))
                .unwrap() as u32)
                * (10u32.pow((4 - index - 1) as u32))
        })
}
