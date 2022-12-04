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
    let file_lines = read_file_lines("day4/input.txt");
    //let file_lines = read_file_lines("day4/example-input.txt");
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

struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn new(range: &str) -> Range {
        let range: (u32, u32) = range
            .split('-')
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Range {
            min: range.0,
            max: range.1,
        }
    }

    fn fully_contains(&self, other: &Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn intersects(&self, other: &Range) -> bool {
        self.min <= other.max && self.max >= other.min
    }
}

fn parse_ranges(file_lines: &[String]) -> impl Iterator<Item = (Range, Range)> + '_ {
    file_lines.iter().filter_map(|line| {
        line.split(',')
            .map(Range::new)
            .collect_tuple::<(Range, Range)>()
    })
}

fn part1(file_lines: &[String]) -> String {
    let num_fully_overlapped = parse_ranges(file_lines)
        .filter(|ranges| ranges.0.fully_contains(&ranges.1) || ranges.1.fully_contains(&ranges.0))
        .count();

    format!("{}", num_fully_overlapped)
}

fn part2(file_lines: &[String]) -> String {
    let num_fully_overlapped = parse_ranges(file_lines)
        .filter(|ranges| ranges.0.intersects(&ranges.1))
        .count();

    format!("{}", num_fully_overlapped)
}
