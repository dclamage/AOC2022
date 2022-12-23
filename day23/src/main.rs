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
    let file_lines = read_file_lines("day23/input.txt");
    //let file_lines = read_file_lines("day23/example-input.txt");
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

fn parse_lines(file_lines: &[String]) -> HashSet<(i64, i64)> {
    let mut map = HashSet::new();
    for (r, line) in file_lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                map.insert((r as i64, c as i64));
            }
        }
    }
    map
}

#[allow(dead_code)]
fn print_map(map: &HashSet<(i64, i64)>) {
    let min_x = map.iter().map(|(r, _)| r).min().unwrap();
    let max_x = map.iter().map(|(r, _)| r).max().unwrap();
    let min_y = map.iter().map(|(_, c)| c).min().unwrap();
    let max_y = map.iter().map(|(_, c)| c).max().unwrap();
    for r in *min_x..=*max_x {
        for c in *min_y..=*max_y {
            if map.contains(&(r, c)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn execute_round(map: &mut HashSet<(i64, i64)>, ordering: usize) -> bool {
    let mut proposed_moves: Vec<((i64, i64), (i64, i64))> = Vec::new();
    let mut proposed_desintations: HashMap<(i64, i64), usize> = HashMap::new();
    for pos in map.iter() {
        let num_elves_n = [(-1, -1), (-1, 0), (-1, 1)]
            .iter()
            .filter(|(dr, dc)| {
                let (r, c) = (pos.0 + dr, pos.1 + dc);
                map.contains(&(r, c))
            })
            .count();

        let num_elves_s = [(1, -1), (1, 0), (1, 1)]
            .iter()
            .filter(|(dr, dc)| {
                let (r, c) = (pos.0 + dr, pos.1 + dc);
                map.contains(&(r, c))
            })
            .count();

        let num_elves_w = [(-1, -1), (0, -1), (1, -1)]
            .iter()
            .filter(|(dr, dc)| {
                let (r, c) = (pos.0 + dr, pos.1 + dc);
                map.contains(&(r, c))
            })
            .count();

        let num_elves_e = [(-1, 1), (0, 1), (1, 1)]
            .iter()
            .filter(|(dr, dc)| {
                let (r, c) = (pos.0 + dr, pos.1 + dc);
                map.contains(&(r, c))
            })
            .count();

        let num_elves = num_elves_n + num_elves_s + num_elves_w + num_elves_e;
        if num_elves == 0 {
            continue;
        }

        let num_elves_ordered = [num_elves_n, num_elves_s, num_elves_w, num_elves_e];
        let directions_ordered = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for ordering_offset in 0..4 {
            let ordering = (ordering + ordering_offset) % 4;
            if num_elves_ordered[ordering] == 0 {
                let offset = directions_ordered[ordering];
                let dest = (pos.0 + offset.0, pos.1 + offset.1);
                proposed_moves.push((*pos, dest));
                *proposed_desintations.entry(dest).or_insert(0) += 1;
                break;
            }
        }
    }

    let mut have_move = false;
    for (pos, dest) in proposed_moves.iter() {
        if proposed_desintations[dest] == 1 {
            map.remove(pos);
            map.insert(*dest);
            have_move = true;
        }
    }

    have_move
}

fn part1(file_lines: &[String]) -> String {
    const PRINT_MAP: bool = false;

    let mut map = parse_lines(file_lines);
    if PRINT_MAP {
        print_map(&map);
    }

    for round in 0..10 {
        let ordering = round % 4;
        execute_round(&mut map, ordering);

        if PRINT_MAP {
            println!("End of round {}", round + 1);
            print_map(&map);
        }
    }

    let min_x = map.iter().map(|(r, _)| r).min().unwrap();
    let max_x = map.iter().map(|(r, _)| r).max().unwrap();
    let min_y = map.iter().map(|(_, c)| c).min().unwrap();
    let max_y = map.iter().map(|(_, c)| c).max().unwrap();
    let total_elves = map.len() as i64;
    let total_empty = (max_x - min_x + 1) * (max_y - min_y + 1) - total_elves;
    total_empty.to_string()
}

fn part2(file_lines: &[String]) -> String {
    let mut map = parse_lines(file_lines);

    let mut round = 0;
    while execute_round(&mut map, round % 4) {
        round += 1;
    }
    round += 1;

    round.to_string()
}
