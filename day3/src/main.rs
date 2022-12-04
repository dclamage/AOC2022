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
    let file_lines = read_file_lines("day3/input.txt");
    //let file_lines = read_file_lines("day3/example_input.txt");
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

fn priority(a: u8) -> u8 {
    if a.is_ascii_lowercase() {
        a - b'a' + 1
    } else {
        a - b'A' + 27
    }
}

fn str_bits(s: &str) -> u64 {
    s.bytes().fold(0u64, |acc, c| acc | 1u64 << priority(c))
}

fn part1(file_lines: &Vec<String>) -> String {
    let priority_total: u64 = file_lines
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| (str_bits(a) & str_bits(b)).trailing_zeros() as u64)
        .sum();

    format!("{}", priority_total)
}

fn part2(file_lines: &Vec<String>) -> String {
    let priority_total: u64 = file_lines
        .iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .fold(u64::MAX, |acc, line| acc & str_bits(line))
                .trailing_zeros() as u64
        })
        .sum();

    format!("{}", priority_total)
}
