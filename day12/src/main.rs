use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let p1 = solve(1);
    println!("{}, {:?}", p1, now.elapsed());

    now = Instant::now();
    let p2 = solve(2);
    println!("{}, {:?}", p2, now.elapsed());
}

fn solve(num_small_visits: usize) -> u32 {
    let mut caves = Vec::<Cave>::new();
    // create all caves
    include_str!("../input.txt").split('\n').for_each(|line| {
        let mut points = line.trim().split('-');
        let start = points.nth(0).unwrap();
        let end = points.nth(0).unwrap();

        if let Some(c) = caves.iter_mut().find(|c| c.name == start) {
            if !c.has_cave(end) {
                c.add_cave(Cave::new(end));
            }
        } else {
            let mut cave = Cave::new(start);
            cave.add_cave(Cave::new(end));
            caves.push(cave);
        }

        if let Some(c) = caves.iter_mut().find(|c| c.name == end) {
            if !c.has_cave(start) {
                c.add_cave(Cave::new(start));
            }
        } else {
            let mut new_cave = Cave::new(end);
            new_cave.add_cave(Cave::new(start));
            caves.push(new_cave);
        }
    });

    let mut path_count = 0u32;
    let start = caves.iter().find(|c| c.name == "start").unwrap();
    start.find_path(
        &Vec::<&str>::new(),
        &caves,
        num_small_visits,
        &mut path_count,
    );
    path_count
}

struct Cave<'a> {
    name: &'a str,
    big: bool,
    connected_caves: Vec<Cave<'a>>,
}

impl<'a> Cave<'a> {
    pub fn new(name: &'a str) -> Self {
        Cave {
            name,
            big: name.to_uppercase() == name,
            connected_caves: Vec::<Cave>::new(),
        }
    }

    pub fn find_path(
        &self,
        visited_caves: &Vec<&str>,
        all_caves: &Vec<Cave>,
        num_small_visits: usize,
        count: &mut u32,
    ) {
        for cave in self.connected_caves.iter().filter(|c| c.name != "start") {
            match cave.name {
                "end" => *count += 1,
                _ => {
                    let next_cave = all_caves.iter().find(|n| n.name == cave.name).unwrap();
                    let mut visited = visited_caves.to_vec();
                    if next_cave.big
                        || (visited.iter().filter(|&&vc| vc == next_cave.name).count()
                            < num_small_visits)
                    {
                        visited.push(next_cave.name);
                        next_cave.find_path(&mut visited, all_caves, num_small_visits, count);
                    }
                }
            }
        }
    }

    pub fn add_cave(&mut self, cave: Cave<'a>) {
        self.connected_caves.push(cave);
    }

    pub fn has_cave(&self, cave_name: &str) -> bool {
        if let Some(_c) = self.connected_caves.iter().find(|c| c.name == cave_name) {
            true
        } else {
            false
        }
    }

    pub fn print(&self) {
        print!("Name: {}, Connected to: ", &self.name);
        for c in self.connected_caves.iter() {
            print!("{}, ", &c.name);
        }
        println!("");
    }
}
