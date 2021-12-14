use std::{fs::write, time::Instant};

fn main() {
    let mut now = Instant::now();
    let p1 = problem_one();
    println!(
        "Problem 1: {}, Completed in {} us",
        p1,
        now.elapsed().as_millis()
    );

    now = Instant::now();
    problem_two();
    println!(
        "Problem 2 Completed in {} us",
        now.elapsed().as_millis()
    );
}

fn problem_one() -> usize {
    let (x_max, y_max, mut grid) = get_input();

    // need x/y and index
    let mut split = include_str!("../folds.txt")
        .split('\n')
        .nth(0)
        .unwrap()
        .split('=');
    let xy = split.nth(0).unwrap().chars().last().unwrap();
    let index = split.nth(0).unwrap().trim().parse::<usize>().unwrap();

    if xy == 'x' {
        fold_grid_x(index, y_max, x_max, &mut grid);
    } else if xy == 'y' {
        fold_grid_y(index, y_max, x_max, &mut grid);
    }

    grid.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|&&b| b).count()
    })
}

fn problem_two() {
    let (mut x_max, mut y_max, mut grid) = get_input();

    // need x/y and index
    let folds: Vec<(char, usize)> = include_str!("../folds.txt")
        .split('\n')
        .map(|s| {
            let mut split = s.trim().split('=');
            (
                split.nth(0).unwrap().chars().last().unwrap(),
                split.nth(0).unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    // items move the difference between fold index and their position
    // but in the other direction
    for i in 0..folds.len() {
        let xy = folds[i].0;
        let index = folds[i].1;

        if xy == 'x' {
            fold_grid_x(index, y_max, x_max, &mut grid);
            x_max = index;
        } else if xy == 'y' {
            fold_grid_y(index, y_max, x_max, &mut grid);
            y_max = index;
        }
    }

    println!("");

    for y in grid.iter() {
        for x in y.iter() {
            if *x {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    println!("");
}

fn fold_grid_y(index: usize, y_max: usize, x_max: usize, grid: &mut Vec<Vec<bool>>) {
    for y in index..y_max {
        for x in 0..x_max {
            if grid[y][x] {
                let diff = y - index;
                grid[index - diff][x] = true;
            }
        }
    }
    grid.resize(index, Vec::<bool>::new());
}

fn fold_grid_x(index: usize, y_max: usize, x_max: usize, grid: &mut Vec<Vec<bool>>) {
    for y in 0..y_max {
        for x in index..x_max {
            if grid[y][x] {
                let diff = x - index;
                grid[y][index - diff] = true;
            }
        }
        grid[y].resize(index, false);
    }
}

fn get_input() -> (usize, usize, Vec<Vec<bool>>) {
    let points: Vec<(usize, usize)> = include_str!("../input.txt")
        .split('\n')
        .map(|s| {
            let mut nums = s.trim().split(',');
            (
                nums.nth(0).unwrap().parse::<usize>().unwrap(),
                nums.nth(0).unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();
    let x_max = points
        .iter()
        .max_by(|(x1, _), (x2, _)| x1.cmp(&x2))
        .unwrap()
        .0
        + 1;
    let y_max = points
        .iter()
        .max_by(|(_, y1), (_, y2)| y1.cmp(&y2))
        .unwrap()
        .1
        + 1;
    let mut grid = Vec::<Vec<bool>>::new();
    for y in 0..y_max {
        grid.push(Vec::<bool>::new());
        for x in 0..x_max {
            grid[y].push(points.contains(&(x, y)));
        }
    }
    (x_max, y_max, grid)
}

fn print_grid(grid: &Vec<Vec<bool>>, name: &str) {
    let mut output = String::new();
    for y in grid.iter() {
        for x in y.iter() {
            output += if *x { "#" } else { " " };
        }
        output += "\n";
    }
    write(name, output).expect("Couldn't write grid.")
}
