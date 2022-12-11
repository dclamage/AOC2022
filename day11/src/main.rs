use std::io::{self, Write};
use std::time::*;
use utility::*;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Parsing
    writeln!(stdout, "Parsing...").unwrap();
    let start_time = Instant::now();
    let file_lines = read_file_lines("day11/input.txt");
    //let file_lines = read_file_lines("day11/example-input.txt");
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

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(usize),
    Multiply(usize),
    MultiplySelf,
}

impl Operation {
    fn execute(&self, x: usize) -> usize {
        match self {
            Operation::Add(x2) => x + x2,
            Operation::Multiply(x2) => x * x2,
            Operation::MultiplySelf => x * x,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    item_list: Vec<usize>,
    operation: Operation,
    test_mod: usize,
    true_monkey: usize,
    false_monkey: usize,
    num_inspections: usize,
}

impl Monkey {
    fn from_lines(file_lines: &[String], current_index: &mut usize) -> Monkey {
        let starting_items_line = &file_lines[*current_index + 1].trim()[16..];
        let operation_line = &file_lines[*current_index + 2].trim()[21..];
        let test_line = &file_lines[*current_index + 3].trim()[19..];
        let true_line = &file_lines[*current_index + 4].trim()[25..];
        let false_line = &file_lines[*current_index + 5].trim()[26..];
        *current_index += 7;

        let item_list: Vec<usize> = starting_items_line
            .split(',')
            .map(|x| x.trim().parse().unwrap())
            .collect();

        let operation = if operation_line == "* old" {
            Operation::MultiplySelf
        } else if operation_line.starts_with('+') {
            Operation::Add(operation_line[2..].parse().unwrap())
        } else {
            Operation::Multiply(operation_line[2..].parse().unwrap())
        };

        let test_mod = test_line.parse().unwrap();
        let true_monkey = true_line.parse().unwrap();
        let false_monkey = false_line.parse().unwrap();

        Monkey {
            item_list,
            operation,
            test_mod,
            true_monkey,
            false_monkey,
            num_inspections: 0,
        }
    }
}

fn parse_monkeys(file_lines: &[String]) -> Vec<Monkey> {
    let mut current_index = 0;
    let mut monkeys: Vec<Monkey> = Vec::new();
    while current_index < file_lines.len() {
        monkeys.push(Monkey::from_lines(file_lines, &mut current_index));
    }
    monkeys
}

fn part1(file_lines: &[String]) -> String {
    let mut monkeys = parse_monkeys(file_lines);

    const NUM_ROUNDS: usize = 20;
    for _ in 0..NUM_ROUNDS {
        for monkey_index in 0..monkeys.len() {
            while !monkeys[monkey_index].item_list.is_empty() {
                monkeys[monkey_index].num_inspections += 1;

                let item = monkeys[monkey_index].item_list.remove(0);
                let item = monkeys[monkey_index].operation.execute(item) / 3;
                let next_monkey = if item % monkeys[monkey_index].test_mod == 0 {
                    monkeys[monkey_index].true_monkey
                } else {
                    monkeys[monkey_index].false_monkey
                };
                monkeys[next_monkey].item_list.push(item);
            }
        }
    }

    let mut inspected_counts: Vec<usize> = monkeys.iter().map(|x| x.num_inspections).collect();
    // Sort highest to lowest
    inspected_counts.sort_by(|a, b| b.cmp(a));

    // Return the product of the top two numbers
    (inspected_counts[0] * inspected_counts[1]).to_string()
}

fn part2(file_lines: &[String]) -> String {
    let mut monkeys = parse_monkeys(file_lines);

    // Working off a modulus of the product of all the test_mods allows
    // for the operations to be congruent to the mod of all the monkey test_mods
    let mod_product = monkeys.iter().map(|x| x.test_mod).product::<usize>();

    const NUM_ROUNDS: usize = 10000;
    for _ in 0..NUM_ROUNDS {
        for monkey_index in 0..monkeys.len() {
            while !monkeys[monkey_index].item_list.is_empty() {
                monkeys[monkey_index].num_inspections += 1;

                let mut item = monkeys[monkey_index].item_list.remove(0);
                item = monkeys[monkey_index].operation.execute(item) % mod_product;
                let next_monkey = if item % monkeys[monkey_index].test_mod == 0 {
                    monkeys[monkey_index].true_monkey
                } else {
                    monkeys[monkey_index].false_monkey
                };
                monkeys[next_monkey].item_list.push(item);
            }
        }
    }

    let mut inspected_counts: Vec<usize> = monkeys.iter().map(|x| x.num_inspections).collect();
    // Sort highest to lowest
    inspected_counts.sort_by(|a, b| b.cmp(a));

    // Return the product of the top two numbers
    (inspected_counts[0] * inspected_counts[1]).to_string()
}
