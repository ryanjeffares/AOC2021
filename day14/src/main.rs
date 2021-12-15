use std::{collections::HashMap, time::Instant};

fn main() {
    let mut now = Instant::now();
    let p1 = solve(10);
    println!("Problem 1: {}, Completed in {:?}", p1, now.elapsed());

    now = Instant::now();
    let p2 = solve(40);
    println!("Problem 2: {}, Completed in {:?}", p2, now.elapsed());
}

fn solve(steps: usize) -> usize {
    let mut input = include_str!("../input.txt").split('\n');

    let polymer_template = input.next().unwrap().trim().as_bytes();
    let mut pair_counts = HashMap::<(char, char), usize>::new();
    let mut char_counts = HashMap::<char, usize>::new();

    input.next(); // consome empty line after template

    let insertion_rules: HashMap<(char, char), char> = input
        .map(|line| {
            let mut split = line.split("->");
            let mut pair = split.next().unwrap().trim().chars();
            let (a, b) = (pair.next().unwrap(), pair.next().unwrap());

            pair_counts.insert((a, b), 0);
            char_counts.insert(a, 0);
            char_counts.insert(b, 0);

            ((a, b), split.next().unwrap().trim().chars().next().unwrap())
        })
        .collect();

    // get initial character counts and existing pairs
    for i in 0..polymer_template.len() {
        let c = polymer_template[i] as char;
        *char_counts.get_mut(&c).unwrap() += 1;
        
        if i < polymer_template.len() - 1 {
            let (a, b) = (c, polymer_template[i + 1] as char);
            *pair_counts.get_mut(&(a, b)).unwrap() += 1;
        }
    }

    for _ in 0..steps {
        let mut temp = pair_counts.clone();

        for rule in insertion_rules.iter() {
            let found_pairs = pair_counts[rule.0];
            // this character will be inserted the amount of times this pair appears            
            *char_counts.get_mut(rule.1).unwrap() += found_pairs;
            
            // find the new pairs that will be made
            let new_pair_1 = (rule.0.0, *rule.1);
            *temp.get_mut(&new_pair_1).unwrap() += found_pairs;

            let new_pair_2 = (*rule.1, rule.0.1);
            *temp.get_mut(&new_pair_2).unwrap() += found_pairs;

            // this pair will be split up so remove all instances of it
            *temp.get_mut(rule.0).unwrap() -= found_pairs;
        }

        pair_counts = temp.clone();
    }

    let max = char_counts
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap()
        .1;

    let min = char_counts
        .iter()
        .min_by(|a, b| a.1.cmp(b.1))
        .unwrap()
        .1;

    max - min
}
