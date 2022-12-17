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
    init_parts(&file_lines);
    let elapsed = start_time.elapsed();
    writeln!(stdout, "Parsing time: {}us\n", elapsed.as_micros()).unwrap();

    // Part 1
    writeln!(stdout, "*********** PART 1 ***********").unwrap();
    let start_time = Instant::now();
    let part1_answer = part1();
    let elapsed = start_time.elapsed();
    writeln!(stdout, "Part 1 answer: {}", part1_answer).unwrap();
    writeln!(stdout, "Part 1 time: {}us\n", elapsed.as_micros()).unwrap();

    // Part 2
    writeln!(stdout, "*********** PART 2 ***********").unwrap();
    let start_time = Instant::now();
    let part2_answer = part2();
    let elapsed = start_time.elapsed();
    writeln!(stdout, "Part 2 answer: {}", part2_answer).unwrap();
    writeln!(stdout, "Part 2 time: {}us", elapsed.as_micros()).unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    id: usize,
    flow_rate: i64,
    connections: Vec<usize>,
    to_valve_moves: Vec<usize>,
}

struct ParsedData {
    valves: Vec<Valve>,
    start_id: usize,
    all_released: u64,
}

thread_local! {
    static PARSED_DATA: RefCell<ParsedData> = RefCell::new(ParsedData{
        valves: Vec::new(),
        start_id: 0,
        all_released: 0,});
    static NUM_MEMOS: RefCell<usize> = RefCell::new(0);
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
            to_valve_moves: Vec::new(),
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
    for (valve_id, valve) in valves.iter().enumerate() {
        assert!(valve.id == valve_id);
    }

    // Find the best next move to get to each valve from each other valve

    // Build a graph in the form that dijkstra's algorithm expects
    let mut graph: Vec<Vec<(usize, i64)>> = Vec::new();
    for valve in valves.iter() {
        let mut adjacency_list = Vec::new();
        for connection in valve.connections.iter() {
            adjacency_list.push((*connection, 1));
        }
        graph.push(adjacency_list);
    }

    let num_valves = valves.len();
    for (starting_valve_id, starting_valve) in valves.iter_mut().enumerate() {
        for to_valve_id in 0..num_valves {
            if starting_valve_id == to_valve_id {
                starting_valve.to_valve_moves.push(0);
                continue;
            }

            let (_, path) = dijkstra(&graph, starting_valve_id, to_valve_id);
            starting_valve.to_valve_moves.push(path[1]);
        }
    }

    (id_map["AA"], valves)
}

fn is_released(has_released: u64, valve_id: usize) -> bool {
    has_released & (1u64 << valve_id) != 0
}

fn set_released(has_released: u64, valve_id: usize) -> u64 {
    has_released | 1u64 << valve_id
}

fn get_worth_visiting_next(current_location: usize, has_released: u64) -> u64 {
    PARSED_DATA.with(|parsed_data| {
        let parsed_data = parsed_data.borrow();
        let valves = &parsed_data.valves;
        let all_released = parsed_data.all_released;
        let current_valve = &valves[current_location];

        let mut not_yet_released = all_released & !has_released;
        let mut worth_visiting_next: u64 = 0;
        while not_yet_released != 0 {
            let valve_id = not_yet_released.trailing_zeros() as usize;
            worth_visiting_next |= 1u64 << current_valve.to_valve_moves[valve_id];
            not_yet_released &= not_yet_released - 1;
        }

        worth_visiting_next
    })
}

#[memoize]
fn find_best_pressure_released(
    current_location: usize,
    minutes_remaining: i64,
    has_released: u64,
) -> i64 {
    PARSED_DATA.with(|parsed_data| {
        let parsed_data = parsed_data.borrow();
        let valves = &parsed_data.valves;
        let all_released = parsed_data.all_released;

        if minutes_remaining == 1 || has_released == all_released {
            return 0;
        }

        let mut best_pressure_released = 0;
        let current_valve = &valves[current_location];
        if current_valve.flow_rate > 0 && !is_released(has_released, current_valve.id) {
            let pressure_released = current_valve.flow_rate * (minutes_remaining - 1)
                + find_best_pressure_released(
                    current_location,
                    minutes_remaining - 1,
                    set_released(has_released, current_valve.id),
                );
            if pressure_released > best_pressure_released {
                best_pressure_released = pressure_released;
            }
        } else {
            if minutes_remaining == 2 {
                return 0;
            }

            // Determine which connections are worth following
            let mut worth_visiting_next: u64 =
                get_worth_visiting_next(current_location, has_released);

            while worth_visiting_next != 0 {
                let connection_id = worth_visiting_next.trailing_zeros() as usize;
                let pressure_released =
                    find_best_pressure_released(connection_id, minutes_remaining - 1, has_released);
                if pressure_released > best_pressure_released {
                    best_pressure_released = pressure_released;
                }
                worth_visiting_next &= worth_visiting_next - 1;
            }
        }
        best_pressure_released
    })
}

fn init_parts(file_lines: &[String]) {
    let (start_id, valves) = parse_valves(file_lines);
    let all_released = valves.iter().fold(0, |acc, v| {
        if v.flow_rate > 0 {
            acc | (1u64 << v.id)
        } else {
            acc
        }
    });
    PARSED_DATA.with(|v| {
        *v.borrow_mut() = ParsedData {
            valves,
            start_id,
            all_released,
        };
    });
}

fn part1() -> String {
    let start_id = PARSED_DATA.with(|parsed_data| parsed_data.borrow().start_id);
    let best_pressure_released = find_best_pressure_released(start_id, 30, 0);
    best_pressure_released.to_string()
}

const NO_LOCATION_GOAL: usize = usize::MAX;

fn find_best_pressure_released_with_partner_entry(
    start_location: usize,
    minutes_remaining: i64,
) -> i64 {
    find_best_pressure_released_with_partner(
        start_location,
        NO_LOCATION_GOAL,
        start_location,
        NO_LOCATION_GOAL,
        minutes_remaining,
        0,
    )
}

fn find_best_pressure_released_with_partner(
    location1: usize,
    location_goal_1: usize,
    location2: usize,
    location_goal_2: usize,
    minutes_remaining: i64,
    has_released: u64,
) -> i64 {
    let all_released = PARSED_DATA.with(|parsed_data| parsed_data.borrow().all_released);

    // With only one minute left, opening a valve won't do anything.
    // Also, if we've already released all valves, there's no point in continuing.
    if minutes_remaining == 1 || has_released == all_released {
        return 0;
    }

    // Cancel the location goal when reaching that location to simplify some logic
    let location_goal_1 = if location_goal_1 == location1 {
        NO_LOCATION_GOAL
    } else {
        location_goal_1
    };
    let location_goal_2 = if location_goal_2 == location2 {
        NO_LOCATION_GOAL
    } else {
        location_goal_2
    };

    // If we picked a location goal but it's already open now, this can't be the most efficient
    if location_goal_1 != NO_LOCATION_GOAL && is_released(has_released, location_goal_1)
        || location_goal_2 != NO_LOCATION_GOAL && is_released(has_released, location_goal_2)
    {
        return 0;
    }

    // If both goals are the same location, this can't be the most efficient
    if location_goal_1 != NO_LOCATION_GOAL && location_goal_1 == location_goal_2 {
        return 0;
    }

    // The two actors are interchangeable, so we can avoid duplicate work by always
    // having the actor with the lower location ID go first.
    if location1 < location2 {
        find_best_pressure_released_with_partner_memoed(
            location1,
            location_goal_1,
            location2,
            location_goal_2,
            minutes_remaining,
            has_released,
        )
    } else {
        find_best_pressure_released_with_partner_memoed(
            location2,
            location_goal_2,
            location1,
            location_goal_1,
            minutes_remaining,
            has_released,
        )
    }
}

#[memoize]
fn find_best_pressure_released_with_partner_memoed(
    location1: usize,
    location_goal_1: usize,
    location2: usize,
    location_goal_2: usize,
    minutes_remaining: i64,
    has_released: u64,
) -> i64 {
    NUM_MEMOS.with(|num_memos| {
        *num_memos.borrow_mut() += 1;
    });

    PARSED_DATA.with(|parsed_data| {
        let parsed_data = parsed_data.borrow();
        let valves = &parsed_data.valves;
        let all_released = parsed_data.all_released;

        let valve1 = &valves[location1];
        let valve2 = &valves[location2];
        let should_open1 = valve1.flow_rate > 0 && !is_released(has_released, location1);
        let should_open2 =
            location1 != location2 && valve2.flow_rate > 0 && !is_released(has_released, location2);

        if !should_open1 && !should_open2 && minutes_remaining <= 2 {
            // 2 minutes isn't long enough to travel somewhere else, open a valve, and have that result in pressure
            return 0;
        }

        let pressure_released_this_round = (if should_open1 { valve1.flow_rate } else { 0 }
            + if should_open2 { valve2.flow_rate } else { 0 })
            * (minutes_remaining - 1);

        let mut has_released = has_released;

        let mut new_locations1 = if should_open1 {
            has_released = set_released(has_released, location1);
            1u64 << location1
        } else if location_goal_1 != NO_LOCATION_GOAL {
            1u64 << location_goal_1
        } else {
            all_released & !has_released
        };

        let new_locations2 = if should_open2 {
            has_released = set_released(has_released, location2);
            1u64 << location2
        } else if location_goal_2 != NO_LOCATION_GOAL {
            1u64 << location_goal_2
        } else {
            all_released & !has_released
        };

        let mut best_pressure_released = 0;
        while new_locations1 != 0 {
            let new_location_goal_1 = new_locations1.trailing_zeros() as usize;
            new_locations1 &= new_locations1 - 1;

            let connection_id_1 = if location1 != new_location_goal_1 {
                valve1.to_valve_moves[new_location_goal_1]
            } else {
                location1
            };

            let mut new_locations2 = new_locations2;
            while new_locations2 != 0 {
                let new_location_goal_2 = new_locations2.trailing_zeros() as usize;
                new_locations2 &= new_locations2 - 1;

                let connection_id_2 = if location2 != new_location_goal_2 {
                    valve2.to_valve_moves[new_location_goal_2]
                } else {
                    location2
                };

                let pressure_released = find_best_pressure_released_with_partner(
                    connection_id_1,
                    new_location_goal_1,
                    connection_id_2,
                    new_location_goal_2,
                    minutes_remaining - 1,
                    has_released,
                );
                if pressure_released > best_pressure_released {
                    best_pressure_released = pressure_released;
                }
            }
        }
        pressure_released_this_round + best_pressure_released
    })
}

fn part2() -> String {
    let start_id = PARSED_DATA.with(|parsed_data| parsed_data.borrow().start_id);
    let best_pressure_released = find_best_pressure_released_with_partner_entry(start_id, 26);

    NUM_MEMOS.with(|num_memos| {
        println!("{} memos", *num_memos.borrow());
    });

    best_pressure_released.to_string()
}
