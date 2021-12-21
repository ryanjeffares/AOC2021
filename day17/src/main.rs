use std::{fs::write, time::Instant};

// reading input from file was honestly too annoying for this one
// so stick your input range here if you're using this
const MIN_X: i32 = 195;
const MAX_X: i32 = 238;
const MIN_Y: i32 = -93;
const MAX_Y: i32 = -67;

fn main() {
    let now = Instant::now();
    let (peak, count) = problem_one();
    println!("Problem 1: {}, Problem 2: {}, Completed in {:?}", peak, count, now.elapsed());
}

// and screw it, im brute forcing
// but i will somewhat reduce the range to check

// there is no need to check any x velocities > max_x of the target
// or y velocities less than the min, obviously

// if x vel is less than the min x of the target
// it can only get there if it won't run out of velocity first
// so x + (x - 1) + (x - 2) ... (x - x) >= MIN_X
// s = x(x + 1) / 2 (s is min_x)
// 2min_x = x^2 + x
// x^2 + x - 2min_x = 0
// x = (-1 + sqrt(1 + 4(-2min_x))) / 2
// round that down, that is the minimum
// possible x values also include the range of the target area itself (assuming the y velocity gets you there in a single step)

fn problem_one() -> (i32, u32) {
    let mut count = ((MAX_X - MIN_X + 1) * ((MIN_Y - MAX_Y).abs() + 1)) as u32;
    let mut peaks = Vec::<i32>::new();

    let sqrt = ((-1 - (4 * (2 * -MIN_X))) as f32).sqrt();    
    let min_x = ((1.0 + sqrt) / 2.0) as i32;
    let possible_x:Vec<i32> = (min_x..MIN_X).collect();
    let mut vels = Vec::<i64>::new();

    for vx in possible_x {
        // values within MIN_X..MAX_X must get there in 1 step - so the y velocity must also be a value within the target 
        // these velocities are already added to the count above

        // in my case there's only 2 values - 20 and 21 - where the x reaches 0 when the projectile is withing the target x - range
        // for these velocities, i cant think of a better way than just looping over a wide range of y values
        if vx == min_x || vx == min_x + 1 {
            for vy in MAX_Y..100 {
                let mut max_y = 0;
                let mut current_vy = vy;
                let mut pos = (0, 0);
                let mut step = 0;
    
                while pos.0 <= MAX_X && pos.1 >= MIN_Y {
                    if step < vx {
                        pos.0 += vx - step;
                        step += 1;
                    }
    
                    pos.1 += current_vy;
    
                    if pos.1 > max_y {
                        max_y = pos.1;
                    }
    
                    if pos.0 >= MIN_X && pos.0 <= MAX_X && pos.1 >= MIN_Y && pos.1 <= MAX_Y {
                        count += 1;
                        peaks.push(max_y);
                        break;
                    }
    
                    current_vy -= 1;
                }
            }
        } else {
            // x velocities between 21 and MIN_X can only get there if the y velocity has dropped us within the target BEFORE vx is 0
            // so step through the x positions, whenever we are within the target range, see what y velocities drop us in there
            let mut step = 0;
            let mut x_pos = 0;
            while x_pos <= MAX_X {
                if step <= vx {
                    x_pos += vx - step;
                }

                if x_pos >= MIN_X && x_pos <= MAX_X {
                    for y in MIN_Y..MAX_Y + 1 {
                        if let Some(vy) = find_y_velocity(step as f64, y as f64) {
                            let v = vx as i64 | ((vy as i64) << 32);
                            if !vels.contains(&v) {
                                count += 1;
                                vels.push(v);
                            }
                        }
                    }
                }

                step += 1;
            }
        }               
    }

    (*peaks.iter().max().unwrap(), count)
}

fn find_y_velocity(step: f64, position: f64) -> Option<i32> {
    let v = (position + ((step.powi(2) + step) / 2.0)) / (step + 1.0);
    if v.trunc() == v {
        Some(v as i32)
    } else {
        None
    }
}

#[allow(dead_code)]
fn print_grid() {
    let mut output = String::new();
    let mut y = 0;
    while y >= -100 {
        for x in 0..250 {
            if x >= MIN_X && x <= MAX_X && y >= MIN_Y && y <= MAX_Y {
                output += "T";
            } else if x == 0 && y == 0 {
                output += "S";
            } else {
                output += ".";
            }
        }
        output += "\n";
        y -= 1;
    }
    write("./grid.txt", output).expect("Couldn't write file");
}
