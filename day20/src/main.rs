use std::{fs::write, time::Instant};

const OFFSETS: [(i32, i32); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn main() {
    let mut now = Instant::now();
    let p1 = solve(2);
    println!("Problem 1: {}, Completed in {:?}", p1, now.elapsed());
    now = Instant::now();
    let p2 = solve(50);
    println!("Problem 2: {}, Completed in {:?}", p2, now.elapsed());
}

fn solve(num_iterations: usize) -> usize {
    let algo = include_str!("../input_algo.txt").trim().as_bytes();

    let mut image = Vec::<Vec<char>>::new();
    let lines: Vec<&str> = include_str!("../input_image.txt").split('\n').collect();

    let input_len = lines.len();
    let input_row_len = lines[0].trim().len();

    let output_len = lines.len() + (num_iterations * 2);
    let output_row_len = lines[0].trim().len() + (num_iterations * 2);

    for y in 0..output_len {
        let mut row = Vec::<char>::new();
        row.resize(output_row_len, '.');

        if y >= num_iterations && y < num_iterations + input_len {
            let line = lines[y - num_iterations].trim();
            for x in 0..row.len() {
                if x >= num_iterations && x < num_iterations + input_row_len {
                    row[x] = line.as_bytes()[x - num_iterations] as char;
                }
            }
        }

        image.push(row);
    }

    // print_image(&image, "./image.txt");

    let mut result_image = Vec::<Vec<char>>::new();
    result_image.resize(output_len, Vec::<char>::new());
    for row in 0..result_image.len() {
        result_image[row].resize(output_row_len, '.');
    }

    for _i in 0..num_iterations {
        enhance_image(&mut result_image, &image, algo);
        image = result_image.to_vec();
        // print_image(&result_image, &format!("./images/image{}.txt", i));
    }

    result_image.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|&&c| c == '#').count()
    })
}

fn enhance_image(image: &mut Vec<Vec<char>>, original: &Vec<Vec<char>>, algo: &[u8]) {
    for row in 0..image.len() {
        for col in 0..image[0].len() {
            let mut res = 0usize;
            for (index, offset) in OFFSETS.iter().enumerate() {
                let x = (col as i32) + offset.0;
                let y = (row as i32) + offset.1;

                if x >= 0 && x < original[0].len() as i32 && y >= 0 && y < original.len() as i32 {
                    if original[y as usize][x as usize] == '#' {
                        res |= 1 << (8 - index);
                    }
                } else {
                    // this is off the edge, from a point along the edge
                    // consider the pixel to be whatever the edge is
                    if original[col][row] == '#' {
                        res |= 1 << (8 - index);
                    }
                }
            }

            image[row][col] = algo[res] as char;
        }
    }
}

#[allow(dead_code)]
fn print_image(image: &Vec<Vec<char>>, filename: &str) {
    let mut output = String::new();
    for row in image.iter() {
        for col in row.iter() {
            output += &col.to_string();
        }
        output += "\n";
    }
    write(filename, output).expect("Couldn't write image");
}
