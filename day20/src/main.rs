use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Write};
use std::time::*;
use utility::*;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Parsing
    writeln!(stdout, "Parsing...").unwrap();
    let start_time = Instant::now();
    let file_lines = read_file_lines("day20/input.txt");
    //let file_lines = read_file_lines("day20/example-input.txt");
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
struct Item {
    value: i64,
    orig_index: usize,
}

fn parse_list(file_lines: &[String]) -> Vec<Item> {
    let mut list = vec![];
    for (i, line) in file_lines.iter().enumerate() {
        let num = line.parse::<i64>().unwrap();
        list.push(Item {
            value: num,
            orig_index: i,
        });
    }
    list
}

fn mix_item(items: &mut Vec<Item>, orig_index: usize) {
    // Find the item's current index which has the original index
    let current_index = items.iter().position(|x| x.orig_index == orig_index).unwrap();

    let value = items[current_index].value;
    if value == 0 {
        return;
    }

    let swap_mod = (items.len() - 1) as i64;
    if value > 0 {
        let num_swaps = (value % swap_mod) as usize;
        for i in 0..num_swaps {
            let index0 = (current_index + i) % items.len();
            let index1 = (current_index + i + 1) % items.len();
            items.swap(index0, index1);
        }
    } else {
        let num_swaps = (-value % swap_mod) as usize;
        for i in 0..num_swaps {
            let index0 = (current_index + items.len() * 2 - i) % items.len();
            let index1 = (current_index + items.len() * 2 - i - 1) % items.len();
            items.swap(index0, index1);
        }
    }
}

#[allow(dead_code)]
fn print_list(items: &[Item]) {
    for item in items.iter() {
        print!("{} ", item.value);
    }
    println!();
}

fn part1(file_lines: &[String]) -> String {
    let mut items = parse_list(file_lines);

    for i in 0..items.len() {
        mix_item(&mut items, i);
    }

    // Find the index of value 0
    let zero_index = items.iter().position(|x| x.value == 0).unwrap();

    let value1000 = items[(zero_index + 1000) % items.len()].value;
    let value2000 = items[(zero_index + 2000) % items.len()].value;
    let value3000 = items[(zero_index + 3000) % items.len()].value;

    let value_sum = value1000 + value2000 + value3000;

    value_sum.to_string()
}

fn part2(file_lines: &[String]) -> String {
    const DECRYPTION_KEY: i64 = 811589153;
    let mut items = parse_list(file_lines);
    for item in items.iter_mut() {
        item.value *= DECRYPTION_KEY;
    }

    for _ in 0..10 {
        for i in 0..items.len() {
            mix_item(&mut items, i);
        }
    }

    // Find the index of value 0
    let zero_index = items.iter().position(|x| x.value == 0).unwrap();

    let value1000 = items[(zero_index + 1000) % items.len()].value;
    let value2000 = items[(zero_index + 2000) % items.len()].value;
    let value3000 = items[(zero_index + 3000) % items.len()].value;

    let value_sum = value1000 + value2000 + value3000;

    value_sum.to_string()
}
