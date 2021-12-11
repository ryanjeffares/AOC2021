use std::{fs::write, time::Instant};

// x, y offset to check
const OFFSETS: [(i8, i8); 8] = [
    (-1, -1),
    ( 0, -1),
    ( 1, -1),
    (-1,  0),
    ( 1,  0),
    (-1,  1),
    ( 0,  1),
    ( 1,  1),
];

fn main() {
    let now = Instant::now();
    let (p1, p2) = solve();
    println!(
        "Problem 1: {}, Problem 2: {}, Completed in {} us.",
        p1,
        p2.unwrap(),
        now.elapsed().as_micros()
    );
}

fn solve() -> (u32, Option<usize>) {
    let mut grid = Vec::<Vec<Octopus>>::new();
    let lines: Vec<&str> = include_str!("../input.txt").split('\n').collect();
    for y in 0..10 {
        grid.push(Vec::<Octopus>::new());
        for x in 0..10 {
            grid[y].push(Octopus {
                x: x as u8,
                y: y as u8,
                light_level: lines[y].chars().nth(x).unwrap().to_digit(10).unwrap() as u8,
                has_flashed: false,
            });
        }
    }

    let mut count = 0u32;
    let mut count_after_100 = 0u32;
    let mut step = 0usize;
    // print_grid(&grid, 0);

    loop {
        step += 1;
        for row in grid.iter_mut() {
            for octopus in row.iter_mut() {
                octopus.step();
            }
        }

        let mut need_to_check = true;
        while need_to_check {
            need_to_check = false;
            for row in 0..10 {
                for col in 0..10 {
                    let octopus = &mut grid[row][col];
                    if octopus.light_level > 9 && !octopus.has_flashed {
                        need_to_check = true;
                        count += 1;
                        octopus.light_level = 0;
                        octopus.has_flashed = true;
                        for offset in OFFSETS.iter() {
                            let x = col as i8 + offset.0;
                            let y = row as i8 + offset.1;
                            if x >= 0 && x < 10 && y >= 0 && y < 10 {
                                let neighbour = &mut grid[y as usize][x as usize];
                                if !neighbour.has_flashed {
                                    neighbour.light_level += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        // print_grid(&grid, step);

        if step == 100 {
            count_after_100 = count;
        }

        if grid.iter().all(|row| row.iter().all(|oct| oct.has_flashed)) {
            return (count_after_100, Some(step));
        }
    }
}

fn print_grid(octopuses: &Vec<Vec<Octopus>>, day: usize) {
    let mut output = String::from("");
    for row in 0..10 {
        for oct in octopuses[row].iter() {
            output += &oct.light_level.to_string();
        }
        if row != 9 {
            output += "\n";
        }
    }
    write(format!("./grids/step{}.txt", &day), output).expect("Couldn't write grid")
}

struct Octopus {
    pub x: u8,
    pub y: u8,
    pub light_level: u8,
    pub has_flashed: bool,
}

impl Octopus {
    pub fn step(&mut self) {
        self.has_flashed = false;
        self.light_level += 1;
    }
}
