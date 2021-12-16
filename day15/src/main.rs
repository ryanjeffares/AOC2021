use std::{
    collections::{HashMap, LinkedList},
    fs::write,
    time::Instant,
};

const GRID_WIDTH: usize = 100;
const VERTICES: usize = GRID_WIDTH * GRID_WIDTH;
const OFFSETS: [(i16, i16); 4] = [
    ( 0, -1), // top
    ( 0,  1),  // bottom
    (-1,  0), // left
    ( 1,  0),  // right
];

fn main() {
    let mut now = Instant::now();
    let p1 = solve(false);
    println!("Problem 1: {}, Completed in {:?}", p1, now.elapsed());

    now = Instant::now();
    let p2 = solve(true);
    println!("Problem 2: {}, Completed in {:?}", p2, now.elapsed());
}

fn solve(big_map: bool) -> usize {
    let mut input: Vec<Vec<u8>> = include_str!("../input.txt")
        .split('\n')
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let width = if big_map { GRID_WIDTH * 5 } else { GRID_WIDTH };

    // resize the map
    if big_map {
        for row in input.iter_mut() {
            row.resize(row.len() * 5, 0);
            for i in 1..5 {
                for j in 0..GRID_WIDTH {
                    let mut value = row[j] + i as u8;
                    if value > 9 {
                        value -= 9;
                    }
                    row[(i * GRID_WIDTH) + j] = value;
                }
            }
        }

        input.resize(input.len() * 5, Vec::<u8>::new());
        for row in GRID_WIDTH..GRID_WIDTH * 5 {
            for col in 0..500 {
                let mut value = input[row % GRID_WIDTH][col] + ((row / GRID_WIDTH) as u8);
                if value > 9 {
                    value -= 9;
                }
                input[row].push(value);
            }
        }
    }

    let mut graph = Graph::new(big_map);

    for row in 0..width {
        for col in 0..width {
            let source = (row * width) + col;
            for offset in OFFSETS.iter() {
                let x = (col as i16) + offset.0;
                let y = (row as i16) + offset.1;

                if x >= 0 && x < width as i16 && y >= 0 && y < width as i16 {
                    let destination = (y as usize * width) + x as usize;
                    graph.add_edge(source, destination, input[y as usize][x as usize]);
                }
            }
        }
    }

    if big_map {
        print_map(&input);
        graph.find_path_big()
    } else {
        graph.find_path()
    }
}

struct Edge {
    source: usize,
    destination: usize,
    risk: u8,
}

struct Graph {
    adjacency_list: HashMap<usize, LinkedList<Edge>>,
}

impl Graph {
    pub fn new(big_map: bool) -> Self {
        let mut graph = Graph {
            adjacency_list: HashMap::<usize, LinkedList<Edge>>::new(),
        };

        let width = if big_map { GRID_WIDTH * 5 } else { GRID_WIDTH };
        for x in 0..width {
            for y in 0..width {
                graph
                    .adjacency_list
                    .insert(x + (y * width), LinkedList::<Edge>::new());
            }
        }
        graph
    }

    pub fn add_edge(&mut self, source: usize, destination: usize, risk: u8) {
        let edge = Edge {
            source,
            destination,
            risk,
        };

        self.adjacency_list
            .get_mut(&source)
            .unwrap()
            .push_front(edge);
    }

    pub fn find_path(&self) -> usize {
        let mut spt = [false; VERTICES];
        let mut risks = [usize::MAX; VERTICES];

        self.dijsktra(risks.as_mut(), spt.as_mut())
    }

    pub fn find_path_big(&self) -> usize {
        let mut spt = Vec::<bool>::new();
        spt.resize(VERTICES * 25, false);
        let mut risks = Vec::<usize>::new();
        risks.resize(VERTICES * 25, usize::MAX);

        self.dijsktra(risks.as_mut_slice(), spt.as_mut_slice())
    }

    fn dijsktra(&self, risks: &mut [usize], spt: &mut [bool]) -> usize {
        let mut pq = Vec::<(usize, usize)>::new();
        risks[0] = 0;
        pq.push((0, 0));
        while !pq.is_empty() {
            let min = pq.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap();
            let pos = pq.iter().position(|v| v == min).unwrap();
            let extracted = pq.remove(pos);

            let extracted_index = extracted.1;
            if !spt[extracted_index] {
                spt[extracted_index] = true;
                let list = &self.adjacency_list[&extracted_index];

                for i in 0..list.len() {
                    let edge = list.iter().nth(i).unwrap();
                    let dest = edge.destination;
                    if !spt[dest] {
                        let dest_risk = risks[extracted_index] + (edge.risk as usize);
                        let current_risk = risks[dest];
                        if current_risk > dest_risk {
                            pq.push((dest_risk, dest));
                            risks[dest] = dest_risk;
                        }
                    }
                }
            }
        }
        *risks.last().unwrap()
    }
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<u8>>) {
    let mut output = String::new();
    for row in map.iter() {
        for col in row.iter() {
            output += &col.to_string();
        }
        output += "\n";
    }
    write("./map.txt", output).expect("Couldn't write map");
}

#[allow(dead_code)]
fn print_graph(graph: &Graph, big_map: bool) {
    let width = if big_map { GRID_WIDTH * 5 } else { GRID_WIDTH };
    for (vertex, list) in graph.adjacency_list.iter() {
        print!("Point {},{}, adjacent to ", vertex % width, vertex / width);
        for l in list.iter() {
            print!(
                "({}, {} risk: {}), ",
                l.destination % width,
                l.destination / width,
                l.risk
            );
        }
        println!("");
    }
}
