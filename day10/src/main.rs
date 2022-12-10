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
    let file_lines = read_file_lines("day10/input.txt");
    //let file_lines = read_file_lines("day10/example-input.txt");
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
enum Command {
    Noop,
    AddX(isize),
}

impl Command {
    fn get_cycles(&self) -> usize {
        match self {
            Command::Noop => 1,
            Command::AddX(_) => 2,
        }
    }

    fn execute(&self, x: isize) -> isize {
        match self {
            Command::Noop => x,
            Command::AddX(x2) => x + x2,
        }
    }
}

fn parse_commands(file_lines: &[String]) -> Vec<Command> {
    let mut commands = Vec::new();
    for line in file_lines {
        if line == "noop" {
            commands.push(Command::Noop);
            continue;
        }

        let (cmd, val) = line.split(' ').collect_tuple().unwrap();
        let val: isize = val.parse().unwrap();
        match cmd {
            "addx" => commands.push(Command::AddX(val)),
            _ => panic!("Unknown command: {}", cmd),
        }
    }
    commands
}

struct Executor {
    commands: Vec<Command>,
    command_index: usize,
    cycle_count: isize,
    cycles_until_next_command: usize,
    x: isize,
    pendingx: isize,
    have_pendingx: bool,
}

impl Executor {
    fn new(commands: Vec<Command>) -> Executor {
        Executor {
            commands,
            command_index: 0,
            cycle_count: 0,
            cycles_until_next_command: 0,
            x: 1,
            pendingx: 0,
            have_pendingx: false,
        }
    }

    fn step(&mut self) -> bool {
        if self.have_pendingx {
            self.x = self.pendingx;
            self.have_pendingx = false;
        }

        if self.command_index >= self.commands.len() {
            return false;
        }

        self.cycle_count += 1;
        if self.cycles_until_next_command == 0 {
            self.cycles_until_next_command = self.commands[self.command_index].get_cycles() - 1;
        } else {
            self.cycles_until_next_command -= 1;
        }

        if self.cycles_until_next_command == 0 {
            self.pendingx = self.commands[self.command_index].execute(self.x);
            self.have_pendingx = true;
            self.command_index += 1;
        }
        true
    }

    fn strength(&self) -> isize {
        self.x * self.cycle_count
    }
}

fn part1(file_lines: &[String]) -> String {
    let commands = parse_commands(file_lines);
    let mut executor = Executor::new(commands);
    let important_cycles: HashSet<isize> = [20, 60, 100, 140, 180, 220].iter().copied().collect();
    let mut important_cycles_sum = 0;
    loop {
        if !executor.step() {
            break;
        }

        if important_cycles.contains(&executor.cycle_count) {
            important_cycles_sum += executor.strength();
        }
    }

    important_cycles_sum.to_string()
}

fn part2(file_lines: &[String]) -> String {
    const CRT_WIDTH: usize = 40;
    const CRT_HEIGHT: usize = 6;
    let commands = parse_commands(file_lines);
    let mut executor = Executor::new(commands);
    let mut crt = vec![' '; CRT_WIDTH * CRT_HEIGHT];
    let mut crt_pos = 0;
    loop {
        if !executor.step() {
            break;
        }

        let x = executor.x;
        let crt_col: isize = (crt_pos % CRT_WIDTH) as isize;
        crt[crt_pos] = if (x - crt_col).abs() <= 1 { 'X' } else { '.' };
        crt_pos += 1;
    }

    let mut output = String::new();
    output.push('\n');
    for row in 0..CRT_HEIGHT {
        for col in 0..CRT_WIDTH {
            output.push(crt[row * CRT_WIDTH + col]);
        }
        output.push('\n');
    }
    output
}
