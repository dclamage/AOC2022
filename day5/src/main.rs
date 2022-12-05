use itertools::Itertools;
use std::io::{self, Write};
use std::time::*;
use utility::*;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Parsing
    writeln!(stdout, "Parsing...").unwrap();
    let start_time = Instant::now();
    let file_lines = read_file_lines("day5/input.txt");
    //let file_lines = read_file_lines("day5/example-input.txt");
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

fn parse_stack_line(line: &str) -> Vec<u8> {
    // Grab every 4th character starting at index 1
    line.bytes().skip(1).step_by(4).collect()
}

#[derive(Debug, Clone)]
struct Stack {
    stack: Vec<u8>,
}

impl Stack {
    fn new() -> Stack {
        Stack { stack: Vec::new() }
    }

    fn push(&mut self, value: u8) {
        self.stack.push(value);
    }

    fn take_multiple(&mut self, count: usize) -> Vec<u8> {
        self.stack.split_off(self.stack.len() - count)
    }

    fn push_multiple(&mut self, values: &[u8]) {
        self.stack.extend(values);
    }

    fn top(&self) -> char {
        self.stack.last().map(|&c| c as char).unwrap_or(' ')
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from_line(line: &str) -> Option<Instruction> {
        let (count, from, to) = line
            .split_ascii_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect_tuple()?;

        Some(Instruction {
            count,
            from: from - 1,
            to: to - 1,
        })
    }

    fn execute(&self, stacks: &mut [Stack], reverse: bool) {
        let mut items = stacks[self.from].take_multiple(self.count);
        if reverse {
            items.reverse();
        }
        stacks[self.to].push_multiple(&items);
    }
}

struct ParsedInput {
    stacks: Vec<Stack>,
    instructions: Vec<Instruction>,
}

impl ParsedInput {
    fn from_lines(lines: &[String]) -> ParsedInput {
        let split_index = lines
            .iter()
            .enumerate()
            .filter(|(_, s)| s.is_empty())
            .map(|(i, _)| i)
            .next()
            .unwrap();

        let stack_lines = lines[..split_index - 1]
            .iter()
            .map(|s| parse_stack_line(s))
            .collect_vec();
        let num_stacks = stack_lines[0].len();
        let mut stacks = vec![Stack::new(); num_stacks];
        for stack_line in stack_lines.iter().rev() {
            for (index, &item) in stack_line.iter().enumerate() {
                if item != b' ' {
                    stacks[index].push(item);
                }
            }
        }

        let instructions = lines[split_index + 1..]
            .iter()
            .filter_map(|s| Instruction::from_line(s))
            .collect_vec();

        ParsedInput {
            stacks,
            instructions,
        }
    }
}

fn do_part(file_lines: &[String], reverse: bool) -> String {
    // Parse the input
    let mut parsed = ParsedInput::from_lines(file_lines);

    // Execute the instructions
    for instruction in parsed.instructions {
        instruction.execute(&mut parsed.stacks, reverse);
    }

    // Create a string from the top of each stack
    parsed.stacks.iter().map(|s| s.top()).collect()
}

fn part1(file_lines: &[String]) -> String {
    do_part(file_lines, true)
}

fn part2(file_lines: &[String]) -> String {
    do_part(file_lines, false)
}
