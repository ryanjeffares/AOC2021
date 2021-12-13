use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let p1 = solve(1);
    println!("{}, {} us", p1, now.elapsed().as_micros());

    now = Instant::now();
    // let p2 = solve(2);
    // println!("{}, {} us", p2, now.elapsed().as_micros());
}

fn solve(num_small_visits: usize) -> u32 {
    let mut caves = Vec::<Cave>::new();
    // create all caves
    include_str!("../input.txt").split('\n').for_each(|line| {
        let mut points = line.trim().split('-');
        let start = points.nth(0).unwrap();
        let end = points.nth(0).unwrap();

        if let Some(c) = caves.iter_mut().find(|c| c.name() == start) {
            if !c.has_cave(end) {
                c.add_cave(Cave::new(end));
            }
        } else {
            let mut cave = Cave::new(start);
            cave.add_cave(Cave::new(end));
            caves.push(cave);
        }

        if let Some(c) = caves.iter_mut().find(|c| c.name() == end) {
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
    let start = caves.iter().find(|c| c.name() == "start").unwrap();
    start.start_find_path(&caves, num_small_visits, &mut path_count);
    path_count
}

struct Cave {
    name: String,
    big: bool,
    connected_caves: Vec<Cave>,
}

impl Cave {
    pub fn new(name: &str) -> Self {
        Cave {
            name: String::from(name),
            big: name.to_uppercase() == name,
            connected_caves: Vec::<Cave>::new(),
        }
    }

    // only call on "start" cave
    pub fn start_find_path(&self, all_caves: &Vec<Cave>, num_small_visits: usize, count: &mut u32) {
        for cc in self.connected_caves.iter() {
            let cave = all_caves.iter().find(|c| cc.name() == c.name()).unwrap();
            let mut visited_caves = vec!["start", cave.name()];
            cave.find_path(&mut visited_caves, &all_caves, num_small_visits, count);
        }
    }

    pub fn find_path<'a, 'b>(
        &self,
        visited_caves: &'a mut Vec<&'b str>,
        all_caves: &'b Vec<Cave>,
        num_small_visits: usize,
        count: &mut u32,
    ) {
        // println!("Count: {}", *count);
        for cave in self.connected_caves.iter() {
            // found the end, increment our count
            if cave.name() == "end" {
                // println!("Found end");
                *count += 1;
            } else {
                if let Some(_c) = visited_caves.iter().find(|&&v| v == cave.name()) {
                    // need to copy the list of visited caves when we start recursing down a new path
                    // and add this cave to that new list of visited caves
                    if cave.big() {
                        let next_cave = all_caves.iter().find(|n| n.name() == cave.name()).unwrap();
                        let mut visited = visited_caves.to_vec();
                        visited.push(next_cave.name());
                        next_cave.find_path(&mut visited, all_caves, num_small_visits, count);
                    } else {
                        if visited_caves.iter().filter(|&&vc| vc == cave.name()).count() < num_small_visits {
                            if cave.name() != "start" && cave.name() != "end" {
                                let next_cave = all_caves.iter().find(|n| n.name() == cave.name()).unwrap();
                                let mut visited = visited_caves.to_vec();
                                visited.push(next_cave.name());
                                next_cave.find_path(&mut visited, all_caves, num_small_visits, count);
                            }
                        }
                    }
                } else {
                    let next_cave = all_caves.iter().find(|n| n.name() == cave.name()).unwrap();                    
                    let mut visited = visited_caves.to_vec();
                    visited.push(next_cave.name());
                    next_cave.find_path(&mut visited, all_caves, num_small_visits, count);
                }
            }
        }
    }

    pub fn add_cave(&mut self, cave: Cave) {
        self.connected_caves.push(cave);
    }

    pub fn has_cave(&self, cave_name: &str) -> bool {
        if let Some(_c) = self.connected_caves.iter().find(|c| c.name() == cave_name) {
            true
        } else {
            false
        }
    }

    pub fn print(&self) {
        print!("Name: {}, Connected to: ", &self.name);
        for c in self.connected_caves.iter() {
            print!("{}, ", &c.name());
        }
        println!("");
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn big(&self) -> bool {
        self.big
    }
}
