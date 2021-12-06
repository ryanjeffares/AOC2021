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

        if include_diagonal {
            print_grid(&grid, "./grid2.txt");
        } else {
            print_grid(&grid, "./grid.txt");
        }

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

    println!("{}", vent_lines.len());

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
where P: AsRef<Path>
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

        VentLine {
            start,
            end,
            direction
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
                let min_x = self.start.x.min(self.end.x);
                let max_x = self.start.x.max(self.end.x);
                point.y == self.start.y && point.x >= min_x && point.x <= max_x
            }
            Direction::Vertical => {
                let min_y = self.start.y.min(self.end.y);
                let max_y = self.start.y.max(self.end.y);
                point.x == self.start.x && point.y >= min_y && point.y <= max_y
            }
            Direction::Diagonal => {
                let slope = (self.end.y - self.start.y) / (self.end.x - self.start.x);
                let y_intercept = self.start.y - (slope * self.start.x);
                // if point.y == (slope * point.x) + y_intercept {
                //     println!("---");
                //     println!(
                //         "Slope is: {}, derived from ({} - {}) / ({} - {})",
                //         slope,
                //         self.end.y,
                //         self.start.y,
                //         self.end.x,
                //         self.start.x
                //     );
                //     println!(
                //         "Y-Intercept is {}, derived from {} - ({} * {})",
                //         y_intercept,
                //         self.start.y,
                //         slope,
                //         self.start.x
                //     );
                //     println!(
                //         "{} == ({} * {}) + {} should return {}",
                //         point.y,
                //         slope,
                //         point.x,
                //         y_intercept,
                //         point.y == (slope * point.x) + y_intercept
                //     );
                // }
                point.y == (slope * point.x) + y_intercept
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
