use std::{collections::HashMap, time::Instant};

type Position = u8;

const TOP: Position = 1;
const TOP_LEFT: Position = 1 << 1;
const TOP_RIGHT: Position = 1 << 2;
const MIDDLE: Position = 1 << 3;
const BOTTOM_LEFT: Position = 1 << 4;
const BOTTOM_RIGHT: Position = 1 << 5;
const BOTTOM: Position = 1 << 6;

const ZERO: Position = TOP | TOP_LEFT | TOP_RIGHT | BOTTOM_LEFT | BOTTOM_RIGHT | BOTTOM;
const ONE: Position = TOP_RIGHT | BOTTOM_RIGHT;
const TWO: Position = TOP | TOP_RIGHT | MIDDLE | BOTTOM_LEFT | BOTTOM;
const THREE: Position = TOP | TOP_RIGHT | MIDDLE | BOTTOM_RIGHT | BOTTOM;
const FOUR: Position = TOP_LEFT | TOP_RIGHT | MIDDLE | BOTTOM_RIGHT;
const FIVE: Position = TOP | TOP_LEFT | MIDDLE | BOTTOM_RIGHT | BOTTOM;
const SIX: Position = TOP | TOP_LEFT | MIDDLE | BOTTOM_LEFT | BOTTOM_RIGHT | BOTTOM;
const SEVEN: Position = TOP | TOP_RIGHT | BOTTOM_RIGHT;
const EIGHT: Position = TOP | TOP_LEFT | TOP_RIGHT | MIDDLE | BOTTOM_LEFT | BOTTOM_RIGHT | BOTTOM;
const NINE: Position = TOP | TOP_LEFT | TOP_RIGHT | MIDDLE | BOTTOM_RIGHT | BOTTOM;

// 1  4  7  8
const UNIQUE_COUNTS: [usize; 4] = [2, 4, 3, 7];
const NUMBER_REPS: [Position; 10] = [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

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
    let mut entries = get_input();
    entries.iter_mut().fold(0, |acc, e| acc + e.decode())
}

struct Entry {
    pub signal_patterns: Vec<String>,
    pub output_value: Vec<String>,
    positions: HashMap<char, Position>,
}

impl Entry {
    pub fn new(signal_patterns: &str, output_value: &str) -> Self {
        Entry {
            signal_patterns: signal_patterns
                .trim()
                .split(' ')
                .map(|s| s.to_string())
                .collect(),
            output_value: output_value
                .trim()
                .split(' ')
                .map(|s| s.to_string())
                .collect(),
            positions: HashMap::<char, Position>::new(),
        }
    }

    pub fn decode(&mut self) -> u32 {
        // known numbers
        let one = self.signal_patterns.iter().find(|s| s.len() == 2).unwrap();
        let four = self.signal_patterns.iter().find(|s| s.len() == 4).unwrap();
        let four_bytes = four.as_bytes();
        let seven = self.signal_patterns.iter().find(|s| s.len() == 3).unwrap();
        let seven_bytes = seven.as_bytes();
        let eight = self.signal_patterns.iter().find(|s| s.len() == 7).unwrap();
        let eight_bytes = eight.as_bytes();

        // can get top segment by comparing 1 and 7
        self.positions.insert(
            seven_bytes[seven.find(|c| !one.contains(c)).unwrap()] as char,
            TOP,
        );

        // can get bottom left by comparing 0/9, 4, and 8
        // only segment thats not in 4, is in 8, and in 1 of the two six segment numbers
        self.positions.insert(
            eight_bytes[eight
                .find(|c| {
                    !four.contains(c)
                        && self
                            .signal_patterns
                            .iter()
                            .filter(|s| s.len() == 6 && s.contains(c))
                            .count()
                            == 2
                })
                .unwrap()] as char,
            BOTTOM_LEFT,
        );

        // can now get bottom - only segment that isnt in 4 or 7 but is in 8, and isnt the bottom left
        self.positions.insert(
            eight_bytes[eight
                .find(|c| {
                    !four.contains(c) && !seven.contains(c) && !self.positions.contains_key(&c)
                })
                .unwrap()] as char,
            BOTTOM,
        );

        // can get top right - only segment that is in 4 and 1, and 2 of the six segment numbers
        self.positions.insert(
            four_bytes[four
                .find(|c| {
                    one.contains(c)
                        && self
                            .signal_patterns
                            .iter()
                            .filter(|s| s.len() == 6 && s.contains(c))
                            .count()
                            == 2
                })
                .unwrap()] as char,
            TOP_RIGHT,
        );

        // can get middle - only segment that is in 4, not already in map, and present in 2 of the six segment numbers
        self.positions.insert(
            four_bytes[four
                .find(|c| {
                    self.signal_patterns
                        .iter()
                        .filter(|s| s.len() == 6 && s.contains(c))
                        .count()
                        == 2
                        && !self.positions.contains_key(&c)
                })
                .unwrap()] as char,
            MIDDLE,
        );

        // can get bottom right - only segment thats in 7, in 4, not top right
        self.positions.insert(
            seven_bytes[seven
                .find(|c| four.contains(c) && !self.positions.contains_key(&c))
                .unwrap()] as char,
            BOTTOM_RIGHT,
        );

        // have top, bottom left, bottom, top right, middle, bottom right
        // top left - only segment thats in 4, not in 1, not middle
        self.positions.insert(
            four_bytes[four
                .find(|c| !one.contains(c) && !self.positions.contains_key(&c))
                .unwrap()] as char,
            TOP_LEFT,
        );

        let mut result = 0u32;
        for i in 0..self.output_value.len() {
            let value: u32 = NUMBER_REPS
                .iter()
                .position(|&x| {
                    x == self.output_value[i]
                        .chars()
                        .fold(0, |acc, c| acc | self.positions[&c])
                })
                .unwrap() as u32;

            if value > 0 {
                result += value * (10u32.pow((self.output_value.len() - i - 1) as u32));
            }
        }

        result
    }
}

fn get_input() -> Vec<Entry> {
    let mut entries = Vec::<Entry>::new();
    include_str!("../input.txt").split('\n').for_each(|s| {
        let mut pair = s.split('|');
        entries.push(Entry::new(pair.nth(0).unwrap(), pair.nth(0).unwrap()))
    });

    entries
}
