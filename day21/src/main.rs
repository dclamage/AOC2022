use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, Write};
use std::time::*;
use utility::*;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Parsing
    writeln!(stdout, "Parsing...").unwrap();
    let start_time = Instant::now();
    let file_lines = read_file_lines("day21/input.txt");
    //let file_lines = read_file_lines("day21/example-input.txt");
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

lazy_static! {
    static ref RE_SCALAR: Regex = Regex::new(r"([a-z]+): (\d+)$").unwrap();
    static ref RE_OP: Regex = Regex::new(r"([a-z]+): ([a-z]+) ([+\-*/]) ([a-z]+)$").unwrap();
}

#[derive(Debug, Clone)]
enum Op {
    Scalar(i64),
    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Div(usize, usize),
    Eq(usize, usize),
}

impl Op {
    fn ids(&self) -> (usize, usize) {
        match self {
            Op::Scalar(_) => (0, 0),
            Op::Add(left, right) => (*left, *right),
            Op::Sub(left, right) => (*left, *right),
            Op::Mul(left, right) => (*left, *right),
            Op::Div(left, right) => (*left, *right),
            Op::Eq(left, right) => (*left, *right),
        }
    }
}

fn parse_lines(file_lines: &[String]) -> (Vec<Op>, HashMap<String, usize>) {
    let mut name_lookup: HashMap<String, usize> = HashMap::new();

    // Populate the name lookup first
    for (id, line) in file_lines.iter().enumerate() {
        if let Some(caps) = RE_SCALAR.captures(line) {
            let name = &caps[1];
            name_lookup.insert(name.to_owned(), id);
        } else if let Some(caps) = RE_OP.captures(line) {
            let name = &caps[1];
            name_lookup.insert(name.to_owned(), id);
        } else {
            panic!("Failed to parse line: {}", line);
        }
    }

    let mut ops = vec![Op::Scalar(0); name_lookup.len()];
    for line in file_lines {
        if let Some(caps) = RE_SCALAR.captures(line) {
            let id = name_lookup[&caps[1]];
            let value = caps[2].parse::<i64>().unwrap();
            ops[id] = Op::Scalar(value);
        } else if let Some(caps) = RE_OP.captures(line) {
            let id = name_lookup[&caps[1]];

            let op_str = &caps[3];
            let left = name_lookup[&caps[2]];
            let right = name_lookup[&caps[4]];
            let op = match op_str {
                "+" => Op::Add(left, right),
                "-" => Op::Sub(left, right),
                "*" => Op::Mul(left, right),
                "/" => Op::Div(left, right),
                _ => panic!("Unknown op: {}", op_str),
            };
            ops[id] = op;
        } else {
            panic!("Failed to parse line: {}", line);
        }
    }
    (ops, name_lookup)
}

fn evaluate(ops: &[Op], id: usize) -> i64 {
    match &ops[id] {
        Op::Scalar(value) => *value,
        Op::Add(left, right) => evaluate(ops, *left) + evaluate(ops, *right),
        Op::Sub(left, right) => evaluate(ops, *left) - evaluate(ops, *right),
        Op::Mul(left, right) => evaluate(ops, *left) * evaluate(ops, *right),
        Op::Div(left, right) => evaluate(ops, *left) / evaluate(ops, *right),
        Op::Eq(left, right) => i64::from(evaluate(ops, *left) == evaluate(ops, *right)),
    }
}

fn part1(file_lines: &[String]) -> String {
    let (ops, name_lookup) = parse_lines(file_lines);
    let root_val = evaluate(&ops, name_lookup["root"]);

    root_val.to_string()
}

fn part2(file_lines: &[String]) -> String {
    let (mut ops, name_lookup) = parse_lines(file_lines);

    // Modify the ops per spec
    let root_id = name_lookup["root"];
    let (left, right) = ops[root_id].ids();
    ops[root_id] = Op::Eq(left, right);

    let humn_id = name_lookup["humn"];
    let mut lower_bound: i64 = 0;
    let mut upper_bound: i64 = 0;
    let mut humn_val: i64 = 1;
    loop {
        ops[humn_id] = Op::Scalar(humn_val);
        let left_val = evaluate(&ops, left);
        let right_val = evaluate(&ops, right);
        match left_val.cmp(&right_val) {
            Ordering::Less => {
                lower_bound = humn_val;
            }
            Ordering::Equal => {
                return humn_val.to_string();
            }
            Ordering::Greater => {
                upper_bound = humn_val;
            }
        }
        if lower_bound == 0 || upper_bound == 0 {
            humn_val *= 2;
        } else {
            humn_val = (lower_bound + upper_bound) / 2;
        }
    }
}
