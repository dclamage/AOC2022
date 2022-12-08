use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use std::time::*;
use utility::*;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Parsing
    writeln!(stdout, "Parsing...").unwrap();
    let start_time = Instant::now();
    let file_lines = read_file_lines("day8/input.txt");
    //let file_lines = read_file_lines("day8/example-input.txt");
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

fn parse_heights(file_lines: &[String]) -> Vec<Vec<i32>> {
    let mut heights = Vec::new();
    for line in file_lines {
        let mut row = Vec::new();
        for height in line.bytes() {
            row.push((height - b'0') as i32);
        }
        heights.push(row);
    }
    heights
}

fn part1(file_lines: &[String]) -> String {
    let heights = parse_heights(file_lines);
    let mut visible_coords = HashSet::new();
    let num_rows = heights.len();
    let num_cols = heights[0].len();
    for row in 0..num_rows {
        let mut tallest: i32 = -1;
        for col in 0..num_cols {
            let h = heights[row][col];
            if h > tallest {
                visible_coords.insert((row, col));
                tallest = h;
            }
        }

        tallest = -1;
        for col in (0..num_cols).rev() {
            let h = heights[row][col];
            if h > tallest {
                visible_coords.insert((row, col));
                tallest = h;
            }
        }
    }

    for col in 0..num_cols {
        let mut tallest: i32 = -1;
        for row in 0..num_rows {
            let h = heights[row][col];
            if h > tallest {
                visible_coords.insert((row, col));
                tallest = h;
            }
        }

        tallest = -1;
        for row in (0..num_rows).rev() {
            let h = heights[row][col];
            if h > tallest {
                visible_coords.insert((row, col));
                tallest = h;
            }
        }
    }

    visible_coords.len().to_string()
}

fn part2(file_lines: &[String]) -> String {
    let heights = parse_heights(file_lines);
    let num_rows = heights.len();
    let num_cols = heights[0].len();

    let mut best_scenic_score = 0;
    for col in 0..num_cols {
        for row in 0..num_rows {
            let mut cur_scenic_score = 1;
            let cur_height = heights[row][col];

            let mut dir_score = 0;
            for other_row in (0..row).rev() {
                dir_score += 1;
                if heights[other_row][col] >= cur_height {
                    break
                }
            }
            cur_scenic_score *= dir_score;

            dir_score = 0;
            for other_row in (row + 1)..num_rows {
                dir_score += 1;
                if heights[other_row][col] >= cur_height {
                    break
                }
            }
            cur_scenic_score *= dir_score;

            dir_score = 0;
            for other_col in (0..col).rev() {
                dir_score += 1;
                if heights[row][other_col] >= cur_height {
                    break
                }
            }
            cur_scenic_score *= dir_score;

            dir_score = 0;
            for other_col in (col + 1)..num_cols {
                dir_score += 1;
                if heights[row][other_col] >= cur_height {
                    break
                }
            }
            cur_scenic_score *= dir_score;

            best_scenic_score = std::cmp::max(best_scenic_score, cur_scenic_score);
        }
    }

    best_scenic_score.to_string()
}
