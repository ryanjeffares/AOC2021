use std::{collections::HashMap, time::Instant};

fn main() {
    let now = Instant::now();
    let (count, value) = solve();
    println!(
        "Problem 1: {}, Problem 2: {}, Completed in {:?}",
        count,
        value,
        now.elapsed()
    );
}

fn solve() -> (u16, u64) {
    let packet = decode(include_str!("../input.txt"));
    // packet.print();
    let count = packet.get_version_count();
    let value = packet.get_value();

    (count, value)
}

fn decode(input: &str) -> Packet {
    let lookup: HashMap<char, &str> = [
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut input_binary_string = String::new();
    let chars = input.chars();
    for c in chars {
        input_binary_string += lookup[&c];
    }

    let input_as_str = input_binary_string.as_str();

    extract_packet(input_as_str).unwrap()
}

fn extract_packet(input: &str) -> Option<Packet> {
    if input.len() < 7 {
        return None;
    }

    let version = u16::from_str_radix(&input[..3], 2).unwrap();
    let type_id = u8::from_str_radix(&input[3..6], 2).unwrap();

    match type_id {
        4 => {
            if input.len() < 11 {
                return None;
            }

            let mut i = 6usize;
            let mut res = String::new();
            loop {
                let slice = &input[i..i + 5];
                let bits = &slice[1..];
                res += bits;
                i += 5;
                if slice.as_bytes()[0] as char == '0' || input.len() < i + 5 {
                    break;
                }
            }

            Some(Packet {
                version,
                type_id,
                length_in_bits: i,
                operator: None,
                literal: Some(u64::from_str_radix(&res, 2).unwrap()),
                sub_packets: Vec::<Packet>::new(),
            })
        }
        _ => {
            let length_id = match input.as_bytes()[6] as char {
                '0' => 15usize,
                '1' => 11usize,
                _ => panic!("Character wasn't '0' or '1'"),
            };

            if input.len() < 7 + length_id {
                return None;
            }

            // if packet length is 15, the next 15 bits represent the number of bits in the sub packet
            // if packet length is 11, the next 11 bits represent the amount of sub packets
            match length_id {
                15 => {
                    let sub_packet_length =
                        usize::from_str_radix(&input[7..7 + length_id], 2).unwrap();

                    if input.len() < 7 + length_id + sub_packet_length {
                        return None;
                    }

                    let mut packet = Packet {
                        version,
                        type_id,
                        length_in_bits: 7 + length_id + sub_packet_length,
                        operator: Some(match type_id {
                            0 => Operator::Sum,
                            1 => Operator::Product,
                            2 => Operator::Minimum,
                            3 => Operator::Maximum,
                            5 => Operator::GreaterThan,
                            6 => Operator::LessThan,
                            7 => Operator::EqualTo,
                            _ => panic!(format!(
                                "type_id {} did not provide a valid operator!",
                                type_id
                            )),
                        }),
                        literal: None,
                        sub_packets: Vec::<Packet>::new(),
                    };

                    let mut slice = &input[7 + length_id..7 + length_id + sub_packet_length];
                    while let Some(p) = extract_packet(slice) {
                        let len = p.length_in_bits;
                        packet.sub_packets.push(p);
                        slice = &slice[len..];
                    }

                    Some(packet)
                }
                11 => {
                    let mut packet = Packet {
                        version,
                        type_id,
                        length_in_bits: 0,
                        operator: Some(match type_id {
                            0 => Operator::Sum,
                            1 => Operator::Product,
                            2 => Operator::Minimum,
                            3 => Operator::Maximum,
                            5 => Operator::GreaterThan,
                            6 => Operator::LessThan,
                            7 => Operator::EqualTo,
                            _ => panic!(format!(
                                "type_id {} did not provide a valid operator!",
                                type_id
                            )),
                        }),
                        literal: None,
                        sub_packets: Vec::<Packet>::new(),
                    };

                    let sub_packet_count =
                        usize::from_str_radix(&input[7..7 + length_id], 2).unwrap();
                    let mut slice = &input[7 + length_id..];
                    for _i in 0..sub_packet_count {
                        if let Some(p) = extract_packet(slice) {
                            let len = p.length_in_bits;
                            packet.sub_packets.push(p);
                            slice = &slice[len..];
                        }
                    }

                    packet.length_in_bits = 7
                        + length_id
                        + packet
                            .sub_packets
                            .iter()
                            .fold(0, |acc, p| acc + p.length_in_bits);

                    Some(packet)
                }
                _ => panic!("length_id was not 11 or 15!"),
            }
        }
    }
}

enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

struct Packet {
    version: u16,
    type_id: u8,
    length_in_bits: usize,
    operator: Option<Operator>,
    literal: Option<u64>,
    sub_packets: Vec<Packet>,
}

impl Packet {
    pub fn get_version_count(&self) -> u16 {
        let mut count = self.version;
        for p in self.sub_packets.iter() {
            count += p.get_version_count();
        }
        count
    }

    pub fn get_value(&self) -> u64 {
        match self.operator {
            Some(Operator::Sum) => self
                .sub_packets
                .iter()
                .fold(0, |acc, p| acc + p.get_value()),
            Some(Operator::Product) => {
                let mut product = 1;
                for p in self.sub_packets.iter() {
                    product *= p.get_value();
                }
                product
            }
            Some(Operator::Minimum) => self
                .sub_packets
                .iter()
                .min_by(|a, b| a.get_value().cmp(&b.get_value()))
                .unwrap()
                .get_value(),
            Some(Operator::Maximum) => self
                .sub_packets
                .iter()
                .max_by(|a, b| a.get_value().cmp(&b.get_value()))
                .unwrap()
                .get_value(),
            Some(Operator::GreaterThan) => {
                let mut iter = self.sub_packets.iter();
                let first = iter.nth(0).unwrap().get_value();
                let second = iter.nth(0).unwrap().get_value();
                if first > second {
                    1
                } else {
                    0
                }
            }
            Some(Operator::LessThan) => {
                let mut iter = self.sub_packets.iter();
                let first = iter.nth(0).unwrap().get_value();
                let second = iter.nth(0).unwrap().get_value();
                if first < second {
                    1
                } else {
                    0
                }
            }
            Some(Operator::EqualTo) => {
                let mut iter = self.sub_packets.iter();
                let first = iter.nth(0).unwrap().get_value();
                let second = iter.nth(0).unwrap().get_value();
                if first == second {
                    1
                } else {
                    0
                }
            }
            None => self.literal.unwrap() as u64,
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("Version: {}", self.version);
        println!("Type ID: {}", self.type_id);
        if let Some(l) = self.literal {
            println!("Literal: {}", l);
        } else {
            for p in self.sub_packets.iter() {
                println!("===");
                println!("Sub packet:");
                println!("Version: {}", p.version);
                if let Some(l) = p.literal {
                    println!("Literal: {}", l);
                } else {
                    println!("Operator:");
                    p.print();
                    println!("===");
                }
            }
        }
    }
}
