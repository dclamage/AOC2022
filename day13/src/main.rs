use itertools::Itertools;
use std::cmp::Ordering;
use std::io::{self, Write};
use std::time::*;
use utility::*;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Parsing
    writeln!(stdout, "Parsing...").unwrap();
    let start_time = Instant::now();
    let file_lines = read_file_lines("day13/input.txt");
    //let file_lines = read_file_lines("day13/example-input.txt");
    let elapsed = start_time.elapsed();
    writeln!(stdout, "Parsing time: {}us\n", elapsed.as_micros()).unwrap();

    // Part 1
    writeln!(stdout, "*********** PART 1 ***********").unwrap();
    let start_time = Instant::now();
    let part1_answer = part1(&file_lines);
    let elapsed = start_time.elapsed();
    writeln!(stdout, "Part 1 answer: {}", part1_answer).unwrap();
    writeln!(stdout, "Part 1 time: {}us\n", elapsed.as_micros()).unwrap();

    // Part 2
    writeln!(stdout, "*********** PART 2 ***********").unwrap();
    let start_time = Instant::now();
    let part2_answer = part2(&file_lines);
    let elapsed = start_time.elapsed();
    writeln!(stdout, "Part 2 answer: {}", part2_answer).unwrap();
    writeln!(stdout, "Part 2 time: {}us", elapsed.as_micros()).unwrap();
}

#[derive(Debug, Clone)]
enum PacketData {
    Empty,
    Value(i64),
    List(Vec<PacketData>),
}

impl PacketData {
    fn from_str(line: &str) -> (usize, PacketData) {
        if line.is_empty() || line.as_bytes()[0] == b']' {
            return (0, PacketData::Empty);
        }
        if line.as_bytes()[0].is_ascii_digit() {
            let mut i = 0;
            while line.as_bytes()[i].is_ascii_digit() {
                i += 1;
            }
            let value = line[0..i].parse::<i64>().unwrap();
            return (i, PacketData::Value(value));
        }

        assert!(line.starts_with('['));

        let mut list: Vec<PacketData> = Vec::new();
        let mut i = 1;
        while i < line.len() {
            let (j, data) = PacketData::from_str(&line[i..]);
            i += j;
            list.push(data);
            if line.as_bytes()[i] == b']' {
                i += 1;
                break;
            }
            assert!(line.as_bytes()[i] == b',');
            i += 1;
        }

        (i, PacketData::List(list))
    }
}

impl PartialEq for PacketData {
    fn eq(&self, other: &PacketData) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for PacketData {}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &PacketData) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &PacketData) -> Ordering {
        match (self, other) {
            (PacketData::Empty, PacketData::Empty) => Ordering::Equal,
            (PacketData::Empty, _) => Ordering::Less,
            (_, PacketData::Empty) => Ordering::Greater,
            (PacketData::Value(a), PacketData::Value(b)) => a.cmp(b),
            (PacketData::Value(a), PacketData::List(_)) => {
                PacketData::List(vec![PacketData::Value(*a)]).cmp(other)
            }
            (PacketData::List(_), PacketData::Value(b)) => {
                self.cmp(&PacketData::List(vec![PacketData::Value(*b)]))
            }
            (PacketData::List(a), PacketData::List(b)) => {
                for (a, b) in a.iter().zip(b.iter()) {
                    match a.cmp(b) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => (),
                    }
                }
                a.len().cmp(&b.len())
            }
        }
    }
}

fn parse_packet_pairs(file_lines: &[String]) -> Vec<(PacketData, PacketData)> {
    let mut packet_pairs: Vec<(PacketData, PacketData)> = Vec::new();
    let mut cur_pair: (PacketData, PacketData) = (PacketData::Empty, PacketData::Empty);
    for (i, line) in file_lines.iter().enumerate() {
        if i % 3 == 2 {
            assert!(line.is_empty());
            continue;
        }
        let (j, data) = PacketData::from_str(line);
        assert_eq!(j, line.len());
        if i % 3 == 0 {
            cur_pair.0 = data;
        } else {
            cur_pair.1 = data;
            packet_pairs.push(cur_pair);
            cur_pair = (PacketData::Empty, PacketData::Empty);
        }
    }
    packet_pairs
}

fn part1(file_lines: &[String]) -> String {
    let packet_pairs = parse_packet_pairs(file_lines);
    let mut correct_pair_sum = 0;
    for (pair_index, pair) in packet_pairs.iter().enumerate() {
        if pair.0 < pair.1 {
            correct_pair_sum += pair_index + 1;
        }
    }

    correct_pair_sum.to_string()
}

fn part2(file_lines: &[String]) -> String {
    let (_, token_one) = PacketData::from_str("[[2]]");
    let (_, token_two) = PacketData::from_str("[[6]]");

    let mut packet_list = Vec::new();
    packet_list.push(token_one.clone());
    packet_list.push(token_two.clone());
    assert!(packet_list[0] == token_one);
    assert!(packet_list[1] == token_two);
    for line in file_lines {
        if line.is_empty() {
            continue;
        }

        let (j, data) = PacketData::from_str(line);
        assert_eq!(j, line.len());
        packet_list.push(data);
    }

    packet_list.sort();

    let token_one_index = packet_list.iter().position(|x| *x == token_one).unwrap() + 1;
    let token_two_index = packet_list.iter().position(|x| *x == token_two).unwrap() + 1;
    let key = token_one_index * token_two_index;
    key.to_string()
}
