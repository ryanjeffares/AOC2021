use std::{fs, time::Instant};

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
        p2, now.elapsed().as_micros()
    );
}

fn problem_one() -> u32 {
    let mut grid = Vec::<Vec<u32>>::new();
    include_str!("../input.txt")
        .split('\n')
        .for_each(|line| grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect()));
    get_low_points(&grid)
        .iter()
        .fold(0, |acc, point| acc + grid[point.1][point.0] + 1)
}

fn problem_two() -> u32 {
    let mut grid = Vec::<Vec<u32>>::new();
    include_str!("../input.txt")
        .split('\n')
        .for_each(|line| grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect()));

    let low_points = get_low_points(&grid);
    let mut basins = Vec::<Vec<(usize, usize)>>::new();
    for point in low_points.iter() {
        let mut basin = Vec::<(usize, usize)>::new();
        recurse_basin(&grid, *point, &mut basin);
        basins.push(basin);
    }

    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut product = 1u32;
    for i in 0..3 {
        product *= basins[i].len() as u32;
    }

    product
}

fn recurse_basin(grid: &Vec<Vec<u32>>, start: (usize, usize), basin: &mut Vec<(usize, usize)>) {
    // check adjacent top
    if start.1 > 0 {
        let point = (start.0, start.1 - 1);
        if grid[point.1][point.0] != 9 && !basin.contains(&point) {
            basin.push(point);
            recurse_basin(grid, point, basin);
        }
    }

    // check adjacent bottom 
    if start.1 < grid.len() - 1 {
        let point = (start.0, start.1 + 1);
        if grid[point.1][point.0] != 9 && !basin.contains(&point) {
            basin.push(point);
            recurse_basin(grid, point, basin);
        }
    }

    // left
    if start.0 > 0 {
        let point = (start.0 - 1, start.1);
        if grid[point.1][point.0] != 9 && !basin.contains(&point) {
            basin.push(point);
            recurse_basin(grid, point, basin);
        }
    }

    if start.0 < grid[0].len() - 1 {
        let point = (start.0 + 1, start.1);
        if grid[point.1][point.0] != 9 && !basin.contains(&point) {
            basin.push(point);
            recurse_basin(grid, point, basin);
        }
    }
}

fn get_low_points(grid: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut res = Vec::<(usize, usize)>::new();
    let y_max = grid.len() - 1;
    for y in 0..grid.len() {
        let x_max = grid[y].len() - 1;
        for x in 0..grid[y].len() {
            let current = grid[y][x];
            let mut top = u32::MAX;
            let mut bottom = u32::MAX;
            let mut left = u32::MAX;
            let mut right = u32::MAX;

            if y > 0 {
                top = grid[y - 1][x];
            }
            if y < y_max {
                bottom = grid[y + 1][x];
            }
            if x > 0 {
                left = grid[y][x - 1];
            }
            if x < x_max {
                right = grid[y][x + 1];
            }

            if current < top && current < bottom && current < left && current < right {
                res.push((x, y));
            }
        }
    }

    res
}

fn print_grid(grid: &Vec<Vec<u32>>) {
    let chars = ['B', 'R', '$', 'X', 'w', 'I', 'v', '+', '~', '.'];
    let mut output = String::default();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            output += &chars[grid[y][x] as usize].to_string();
        }
        output += "\n";
    }
    fs::write("./grid.txt", output).expect("Problem writing grid.")
}
