use std::{collections::HashMap, time::Instant};

const OPENERS: [char; 4] = ['(', '[', '{', '<'];
const CLOSERS: [char; 4] = [')', ']', '}', '>'];

fn main() {
    let now = Instant::now();
    let (p1, p2) = problem_one();
    println!(
        "Problem 1: {}, Problem 2: {}, Completed in {} us.",
        p1,
        p2,
        now.elapsed().as_micros()
    )
}

fn problem_one() -> (u32, usize) {
    let mut open_close_pairs = HashMap::<char, char>::new();
    open_close_pairs.insert('(', ')');
    open_close_pairs.insert('[', ']');
    open_close_pairs.insert('{', '}');
    open_close_pairs.insert('<', '>');

    let mut scores = HashMap::<char, u32>::new();
    scores.insert(')', 3);
    scores.insert(']', 57);
    scores.insert('}', 1197);
    scores.insert('>', 25137);

    let corrupted_lines: Vec<&str> = include_str!("../input.txt")
        .split('\n')
        .map(|l| l.trim())
        .collect();


    let mut sum = 0;
    let mut fix_scores = Vec::<usize>::new();
    for line in corrupted_lines.iter() {
        let (error_char, score) = find_error(line, &open_close_pairs);

        if let Some(c) = error_char {
            sum += scores[&c];
        } else {
            fix_scores.push(score);
        }
    }

    fix_scores.sort();    

    (sum, fix_scores[fix_scores.len() / 2])
}

// returns the error character, and the score for fixing the line
// if it was a corrupted line, this will return a character and 0
// if it was an incomplete line, this will return None and the score
fn find_error(line: &str, pairs: &HashMap<char, char>) -> (Option<char>, usize) {
    let mut stack = Vec::<char>::new();
    for c in line.chars() {
        if OPENERS.contains(&c) {
            stack.push(c);
        }
        if CLOSERS.contains(&c) {
            if c == pairs[stack.last().unwrap()] {
                stack.pop();
            } else {
                // this is the error
                return (Some(c), 0);
            }
        }
    }

    let mut score = 0;
    while !stack.is_empty() {
        score *= 5;
        score += CLOSERS.iter().position(|p| p == &pairs[stack.last().unwrap()]).unwrap() + 1;
        stack.pop();
    }
    
    (None, score)
}
