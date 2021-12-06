use std::{
    fs::File,
    io::{self, BufRead, Lines},
    path::Path,
    time::Instant,
};

fn main() {
    let mut now = Instant::now();
    let p1 = solve(false);
    println!(
        "Problem 1: {}, Completed in {} us.",
        p1,
        now.elapsed().as_micros()
    );

    now = Instant::now();
    let p2 = solve(true);
    println!(
        "Problem 2: {}, Completed in {} us.",
        p2,
        now.elapsed().as_micros()
    );
}

fn solve(include_diagonal: bool) -> u32 {
    if let Ok(lines) = read_lines("./input.txt") {
        let (vent_lines, mut grid) = get_input(lines, include_diagonal);

        // println!("{}", vent_lines.len());
        // find the overlaps
        vent_lines.iter().for_each(|vl| {
            grid.iter_mut().for_each(|row| {
                row.iter_mut().for_each(|point| {
                    if vl.check_overlap(point) {
                        point.overlaps += 1
                    }
                })
            })
        });

        // if include_diagonal {
        //     print_grid(&grid, "./grid2.txt");
        // } else {
        //     print_grid(&grid, "./grid.txt");
        // }

        // get amount of points with >= 2 overlaps
        let mut sum = 0u32;
        grid.iter()
            .for_each(|row| sum += row.iter().filter(|vp| vp.overlaps >= 2).count() as u32);
        return sum;
    }

    0
}

fn get_input(
    lines: Lines<io::BufReader<File>>,
    include_diagonal: bool,
) -> (Vec<VentLine>, Vec<Vec<VentPoint>>) {
    let mut vent_lines = Vec::<VentLine>::new();
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = 0i32;
    let mut max_y = 0i32;
    for l in lines {
        if let Ok(line) = l {
            let mut points = line.trim().split("->");

            let start = points.nth(0).unwrap();
            let mut start_points = start.trim().split(',');
            let x1 = start_points.nth(0).unwrap().parse::<i32>().unwrap();
            let y1 = start_points.nth(0).unwrap().parse::<i32>().unwrap();

            let end = points.nth(0).unwrap();
            let mut end_points = end.trim().split(',');
            let x2 = end_points.nth(0).unwrap().parse::<i32>().unwrap();
            let y2 = end_points.nth(0).unwrap().parse::<i32>().unwrap();

            if include_diagonal {
                vent_lines.push(VentLine::new(
                    VentPoint::new(x1, y1),
                    VentPoint::new(x2, y2),
                ));
            } else {
                if x1 == x2 || y1 == y2 {
                    vent_lines.push(VentLine::new(
                        VentPoint::new(x1, y1),
                        VentPoint::new(x2, y2),
                    ));
                }
            }

            min_x = min_x.min(x1.min(x2));
            min_y = min_y.min(y1.min(y1));

            max_x = max_x.max(x1.max(x2));
            max_y = max_y.max(y1.max(y2));
        }
    }

    let mut grid = Vec::<Vec<VentPoint>>::new();
    for y in 0..max_y - min_y {
        grid.push(Vec::<VentPoint>::new());
        for x in 0..max_x - min_x {
            grid[y as usize].push(VentPoint::new(x + min_x, y + min_y));
        }
    }

    (vent_lines, grid)
}

fn print_grid<P>(grid: &Vec<Vec<VentPoint>>, filename: P)
where
    P: AsRef<Path>,
{
    let mut output: String = String::new();
    for row in grid.iter() {
        for point in row.iter() {
            if point.overlaps > 0 {
                output += format!("{}", point.overlaps).as_str();
            } else {
                output += ".";
            }
        }
        output += "\n";
    }

    std::fs::write(filename, output).expect("Couldn't write grid.")
}

#[derive(PartialEq)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
}

struct VentPoint {
    pub x: i32,
    pub y: i32,
    pub overlaps: u32,
}

impl VentPoint {
    pub fn new(x: i32, y: i32) -> Self {
        VentPoint { x, y, overlaps: 0 }
    }
}

struct VentLine {
    pub start: VentPoint,
    pub end: VentPoint,
    pub direction: Direction,
    max_x: i32,
    max_y: i32,
    min_x: i32,
    min_y: i32,
}

impl VentLine {
    pub fn new(start: VentPoint, end: VentPoint) -> Self {
        let direction = if start.x == end.x {
            Direction::Vertical
        } else if start.y == end.y {
            Direction::Horizontal
        } else {
            Direction::Diagonal
        };

        let max_x = start.x.max(end.x);
        let max_y = start.y.max(end.y);
        let min_x = start.x.min(end.x);
        let min_y = start.y.min(end.y);

        VentLine {
            start,
            end,
            direction,
            max_x,
            max_y,
            min_x,
            min_y,
        }
    }

    pub fn print(&self) {
        println!(
            "{},{} -> {},{}",
            self.start.x, self.start.y, self.end.x, self.end.y,
        )
    }

    pub fn check_overlap(&self, point: &VentPoint) -> bool {
        match self.direction {
            Direction::Horizontal => {
                point.y == self.start.y && is_between(point.x, self.min_x, self.max_x)
            }
            Direction::Vertical => {
                point.x == self.start.x && is_between(point.y, self.min_y, self.max_y)
            }
            Direction::Diagonal => {
                let slope = (self.end.y - self.start.y) / (self.end.x - self.start.x);
                let y_intercept = self.start.y - (slope * self.start.x);
                is_between(point.x, self.min_x, self.max_x)
                    && is_between(point.y, self.min_y, self.max_y)
                    && point.y == (slope * point.x) + y_intercept
            }
        }
    }
}

fn is_between(num: i32, min: i32, max: i32) -> bool {
    num <= max && num >= min
}

fn read_lines<P>(filename: P) -> io::Result<Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
