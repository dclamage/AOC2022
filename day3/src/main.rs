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

fn priority(a: char) -> i64 {
    if a.is_ascii_lowercase() {
        a as i64 - 'a' as i64 + 1
    } else {
        a as i64 - 'A' as i64 + 27
    }
}

fn str_bits(s: &str) -> u64 {
    s.chars().fold(0, |acc, c| acc | 1 << priority(c))
}

fn part1(file_lines: &Vec<String>) -> String {
    let mut priority_total: i64 = 0;
    for line in file_lines.iter() {
        let (left, right) = line.split_at(line.len() / 2);
        let common_bits = str_bits(left) & str_bits(right);
        assert!(common_bits != 0 && common_bits.count_ones() == 1);
        priority_total += common_bits.trailing_zeros() as i64;
    }

    format!("{}", priority_total)
}

fn part2(file_lines: &Vec<String>) -> String {
    let mut priority_total: i64 = 0;
    let mut common_line_bits = u64::MAX;
    for (line_index, line) in file_lines.iter().enumerate() {
        common_line_bits &= str_bits(line);

        if line_index % 3 == 2 {
            assert!(common_line_bits != 0 && common_line_bits.count_ones() == 1);
            priority_total += common_line_bits.trailing_zeros() as i64;
            common_line_bits = u64::MAX;
        }
    }

    format!("{}", priority_total)
}
