fn main() {
    problem_one();
}

fn problem_one() {
    let numbers_list: Vec<&str> = include_str!("../input.txt").split('\n').collect();

    let mut first = Pair::default();
    first.nest_level = 1;
    make_number(&numbers_list[0], &mut first, 0);
    first.print();

    let mut second = Pair::default();    
    second.nest_level = 1;
    make_number(&numbers_list[1], &mut second, 0);
    second.print();

    // let mut first_sum = Pair {
        // left_number: None,
        // left_pair: Some(Box::new(first)),
        // right_number: None,
        // right_pair: Some(Box::new(second)),
        // nest_level: 0,
    // };
// 
    // first_sum.reduce();

    // let mut i = 2;
    // while i < numbers_list.len() {}
}

fn make_number(line: &str, pair: &mut Pair, mut index: usize) -> usize {
    let bytes = line.as_bytes();
    let mut stack = 0;

    loop {
        let c = bytes[index] as char;
        match c {
            '[' => {
                stack += 1;

                let next = bytes[index + 1] as char;
                if next == '[' {
                    let mut left = Pair::default();
                    left.nest_level = pair.nest_level + 1;
                    index = make_number(&line, &mut left, index + 1);
                    pair.left_pair = Some(Box::new(left));
                } else {
                    let left = next.to_digit(10).unwrap() as u8;
                    pair.left_number = Some(left);
                    index += 2;
                }
            }
            ']' => {
                stack -= 1;
                index += 1;
            }
            ',' => {
                let next = bytes[index + 1] as char;
                if next == '[' {
                    let mut right_pair = Pair::default();
                    right_pair.nest_level = pair.nest_level + 1;
                    index = make_number(&line, &mut right_pair, index + 1);
                    pair.right_pair = Some(Box::new(right_pair));
                } else {
                    let right = next.to_digit(10).unwrap() as u8;
                    pair.right_number = Some(right);
                    index += 2;
                }
            }
            _ => {
                let left = c.to_digit(10).unwrap() as u8;
                pair.left_number = Some(left);
                index += 1;
            }
        }

        if stack == 0 {
            break;
        }
    }

    index
}

struct Pair {
    left_number: Option<u8>,
    left_pair: Option<Box<Pair>>,
    right_number: Option<u8>,
    right_pair: Option<Box<Pair>>,
    nest_level: u8,
}

impl Default for Pair {
    fn default() -> Self {
        Pair {
            left_number: None,
            left_pair: None,
            right_number: None,
            right_pair: None,
            nest_level: 0,
        }
    }
}

impl Pair {
    // explode - number nested withing 4 pairs
    // split - regular number 10 or greater
    fn reduce(&mut self) {
        // check left for 4 nested pair
        self.try_explode();
    }    

    fn try_explode(&mut self) -> bool {
        // check if left pair is a pair of numbers and is nested enough
        if self.left_pair.is_some() {
            let left = self.left_pair.as_mut().unwrap();
            if left.nest_level >= 4 && left.left_number.is_some() && left.right_number.is_some() {
                // if right is a number, add to it
                if self.right_number.is_some() {
                    *self.right_number.as_mut().unwrap() += left.right_number.unwrap();
                } else {
                    
                }
            }
        }        

        false
    }

    fn find_nested_pair(&mut self) -> Option<&mut Self> {
        if self.left_number.is_some() && self.right_number.is_some() {
            if self.nest_level >= 4 {
                return Some(self);
            }
        } else {
            return self.find_nested_pair();
        }

        None
    }

    #[allow(dead_code)]
    fn print(&self) {
        print!("Pair, Nest Level {}: [", self.nest_level);

        if let Some(n) = self.left_number {
            print!("Left Number: {} ", n);
        }

        if let Some(p) = &self.left_pair {
            p.print();
        }

        print!(", ");

        if let Some(n) = self.right_number {
            print!("Right number: {} ", n);
        }

        if let Some(p) = &self.right_pair {
            p.print();
        }

        print!("]");
    }
}
