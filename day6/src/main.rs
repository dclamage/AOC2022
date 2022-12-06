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
    let file_lines = read_file_lines("day6/input.txt");
    //let file_lines = read_file_lines("day6/example-input.txt");
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

fn find_marker(line: &str, unique_len: usize) -> usize {
    let line = line.bytes().enumerate().collect_vec();
    for window in line.windows(unique_len) {
        if window.iter().copied().map(|(_, c)| c).unique().count() == unique_len {
            return window[unique_len - 1].0 as usize;
        }
    }

    0
}

fn part1(file_lines: &[String]) -> String {
    let marker_index = find_marker(&file_lines[0], 4);
    format!("{}", marker_index + 1)
}

fn part2(file_lines: &[String]) -> String {
    let marker_index = find_marker(&file_lines[0], 14);
    format!("{}", marker_index + 1)
}
