use itertools::Itertools;
use lazy_static::lazy_static;
use memoize::memoize;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};
use std::time::*;
use utility::*;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Parsing
    writeln!(stdout, "Parsing...").unwrap();
    let start_time = Instant::now();
    let file_lines = read_file_lines("day16/input.txt");
    //let file_lines = read_file_lines("day16/example-input.txt");
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    id: usize,
    flow_rate: i64,
    connections: Vec<usize>,
}

thread_local! {
    static VALVES: RefCell<Vec<Valve>> = RefCell::new(Vec::new());
    static ALL_RELEASED: RefCell<u64> = RefCell::new(0);
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"Valve ([A-Z]+) has flow rate=([0-9]+); tunnels? leads? to valves? (.*)")
            .unwrap();
}

impl Valve {
    fn from_string(id_map: HashMap<String, usize>, s: &str) -> Self {
        let cap = RE.captures(s).unwrap();
        let id = id_map[&cap[1].to_owned()];
        let flow_rate = cap[2].parse::<i64>().unwrap();
        let connections = cap[3].split(", ").map(|s| id_map[s]).collect_vec();
        Self {
            id,
            flow_rate,
            connections,
        }
    }

    fn parse_name(s: &str) -> String {
        let cap = RE.captures(s).unwrap();
        cap[1].to_owned()
    }
}

fn parse_valves(file_lines: &[String]) -> (usize, Vec<Valve>) {
    let mut id_map = HashMap::new();
    let mut valves = Vec::new();
    for (i, line) in file_lines.iter().enumerate() {
        let name = Valve::parse_name(line);
        id_map.insert(name, i);
    }
    for line in file_lines {
        valves.push(Valve::from_string(id_map.clone(), line));
    }
    (id_map["AA"], valves)
}

fn is_released(has_released: u64, valve_id: usize) -> bool {
    has_released & (1 << valve_id) != 0
}

fn set_released(has_released: u64, valve_id: usize) -> u64 {
    has_released | 1 << valve_id
}

#[memoize]
fn find_best_pressure_released(
    current_location: usize,
    minutes_remaining: i64,
    has_released: u64,
    all_released: u64,
) -> i64 {
    if minutes_remaining == 0 || has_released == all_released {
        return 0;
    }

    VALVES.with(|valves| {
        let valves = valves.borrow();
        let mut best_pressure_released = 0;
        let current_valve = &valves[current_location];
        if current_valve.flow_rate > 0 && !is_released(has_released, current_valve.id) {
            let pressure_released = current_valve.flow_rate * (minutes_remaining - 1)
                + find_best_pressure_released(
                    current_location,
                    minutes_remaining - 1,
                    set_released(has_released, current_valve.id),
                    all_released,
                );
            if pressure_released > best_pressure_released {
                best_pressure_released = pressure_released;
            }
        }
        for &connection_id in &current_valve.connections {
            let pressure_released = find_best_pressure_released(
                connection_id,
                minutes_remaining - 1,
                has_released,
                all_released,
            );
            if pressure_released > best_pressure_released {
                best_pressure_released = pressure_released;
            }
        }
        best_pressure_released
    })
}

fn part1(file_lines: &[String]) -> String {
    let (start_id, valves) = parse_valves(file_lines);
    let all_released = valves.iter().fold(0, |acc, v| {
        if v.flow_rate > 0 {
            acc | (1 << v.id)
        } else {
            acc
        }
    });
    VALVES.with(|v| *v.borrow_mut() = valves.clone());

    let best_pressure_released = find_best_pressure_released(start_id, 30, 0, all_released);
    best_pressure_released.to_string()
}

#[memoize]
fn find_best_pressure_released_with_partner(
    location1: usize,
    location2: usize,
    minutes_remaining: i64,
    has_released: u64,
    all_released: u64,
) -> i64 {
    if minutes_remaining == 0 || has_released == all_released {
        return 0;
    }

    VALVES.with(|valves| {
        let valves = valves.borrow();
        let mut best_pressure_released = 0;
        let valve1 = &valves[location1];
        let valve2 = &valves[location2];
        for move1 in 0..=valve1.connections.len() {
            if move1 == 0 && (valve1.flow_rate == 0 || is_released(has_released, valve1.id)) {
                continue;
            }

            let mut new_location1 = location1;
            let mut move1_has_released = has_released;
            let mut move1_pressure_released_this_round = 0;
            if move1 == 0 {
                move1_pressure_released_this_round = valve1.flow_rate * (minutes_remaining - 1);
                move1_has_released = set_released(has_released, valve1.id);
            } else {
                new_location1 = valve1.connections[move1 - 1];
            }

            for move2 in 0..=valve2.connections.len() {
                if move2 == 0
                    && (valve2.flow_rate == 0 || is_released(move1_has_released, valve2.id))
                {
                    continue;
                }

                let mut new_location2 = location2;
                let mut move2_has_released = move1_has_released;
                let mut move2_pressure_released_this_round = 0;
                if move2 == 0 {
                    move2_pressure_released_this_round = valve2.flow_rate * (minutes_remaining - 1);
                    move2_has_released = set_released(move1_has_released, valve2.id);
                } else {
                    new_location2 = valve2.connections[move2 - 1];
                }

                let pressure_released = move1_pressure_released_this_round
                    + move2_pressure_released_this_round
                    + find_best_pressure_released_with_partner(
                        new_location1,
                        new_location2,
                        minutes_remaining - 1,
                        move2_has_released,
                        all_released,
                    );

                if pressure_released > best_pressure_released {
                    best_pressure_released = pressure_released;
                }
            }
        }
        best_pressure_released
    })
}

fn part2(file_lines: &[String]) -> String {
    let (start_id, valves) = parse_valves(file_lines);
    let all_released = valves.iter().fold(0, |acc, v| {
        if v.flow_rate > 0 {
            acc | (1 << v.id)
        } else {
            acc
        }
    });
    VALVES.with(|v| *v.borrow_mut() = valves.clone());

    let best_pressure_released =
        find_best_pressure_released_with_partner(start_id, start_id, 26, 0, all_released);
    best_pressure_released.to_string()
}
