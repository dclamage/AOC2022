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
    prev: usize,
    next: usize,
}

fn parse_list(file_lines: &[String]) -> Vec<Item> {
    let mut list = vec![];
    for (i, line) in file_lines.iter().enumerate() {
        let num = line.parse::<i64>().unwrap();
        list.push(Item {
            value: num,
            prev: (i + file_lines.len() - 1) % file_lines.len(),
            next: (i + 1) % file_lines.len(),
        });
    }
    list
}

fn mix_item(items: &mut [Item], index: usize) {
    let item_value = items[index].value;

    if item_value == 0 {
        return;
    }

    let new_index = if item_value > 0 {
        // Find how many places to move the item to the right
        let move_right = item_value as usize;

        // Traverse the list to find the new location for the item
        let mut new_index = index;
        for _ in 0..move_right {
            new_index = items[new_index].next;
        }
        new_index
    } else {
        // Find how many places to move the item to the left
        // (add 1 to the value to account for the item inserting to the right of the new index)
        let move_left = (-item_value) as usize + 1;

        // Traverse the list to find the new location for the item
        let mut new_index = index;
        for _ in 0..move_left {
            new_index = items[new_index].prev;
        }
        new_index
    };

    if new_index == index {
        return;
    }

    // Remove the item from the list
    let prev = items[index].prev;
    let next = items[index].next;
    items[prev].next = next;
    items[next].prev = prev;

    // Insert the item into the new index
    let next = items[new_index].next;
    items[new_index].next = index;
    items[next].prev = index;
    items[index].prev = new_index;
    items[index].next = next;
}

#[allow(dead_code)]
fn print_list(items: &[Item], start_index: usize) {
    let mut index = start_index;
    for _ in 0..items.len() {
        print!("{} ", items[index].value);
        index = items[index].next;
    }
    println!();
}

fn get_index_from(items: &[Item], start_index: usize, length_from_index: usize) -> usize {
    let length_from_index = length_from_index % items.len();
    let mut index = start_index;
    for _ in 0..length_from_index {
        index = items[index].next;
    }
    index
}

fn part1(file_lines: &[String]) -> String {
    let mut items = parse_list(file_lines);
    for i in 0..items.len() {
        mix_item(&mut items, i);
    }

    // Find the index of value 0
    let zero_index = items.iter().position(|x| x.value == 0).unwrap();

    let index1000 = get_index_from(&items, zero_index, 1000);
    let index2000 = get_index_from(&items, index1000, 1000);
    let index3000 = get_index_from(&items, index2000, 1000);

    let value_sum = items[index1000].value
        + items[index2000].value
        + items[index3000].value;

    value_sum.to_string()
}

fn part2(file_lines: &[String]) -> String {
    "".to_string()
}
