use itertools::Itertools;
use std::collections::HashSet;
use std::io::{self, Write};
use std::time::*;
use utility::*;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Parsing
    writeln!(stdout, "Parsing...").unwrap();
    let start_time = Instant::now();
    let file_lines = read_file_lines("day14/input.txt");
    //let file_lines = read_file_lines("day14/example-input.txt");
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

fn parse_map(file_lines: &[String]) -> HashSet<(i64, i64)> {
    let mut map = HashSet::new();
    for line in file_lines {
        let positions = line.split(" -> ").collect_vec();
        let from = positions[0].split(',').collect_vec();
        let mut from = (from[0].parse::<i64>().unwrap(), from[1].parse::<i64>().unwrap());
        for to in positions.iter() {
            let to = to.split(',').collect_vec();
            let to = (to[0].parse::<i64>().unwrap(), to[1].parse::<i64>().unwrap());
            if from.0 == to.0 {
                if from.1 < to.1 {
                    for y in from.1..=to.1 {
                        map.insert((from.0, y));
                    }
                } else {
                    for y in to.1..=from.1 {
                        map.insert((from.0, y));
                    }
                }
            } else {
                assert_eq!(from.1, to.1, "Diagonal lines not supported");
                if from.0 < to.0 {
                    for x in from.0..=to.0 {
                        map.insert((x, from.1));
                    }
                } else {
                    for x in to.0..=from.0 {
                        map.insert((x, from.1));
                    }
                }
            }
            from = to;
        }
    }

    map
}

fn get_map_minmax(map: &HashSet<(i64, i64)>) -> ((i64, i64), (i64, i64)) {
    let min_x = map.iter().map(|(x, _)| x).min().unwrap();
    let max_x = map.iter().map(|(x, _)| x).max().unwrap();
    let min_y = map.iter().map(|(_, y)| y).min().unwrap();
    let max_y = map.iter().map(|(_, y)| y).max().unwrap();
    ((*min_x, *min_y), (*max_x, *max_y))
}

fn drop_sand(map: &mut HashSet<(i64, i64)>, max_y: i64, x: i64, y: i64) -> bool {
    let mut sand_pos = (x, y);
    while sand_pos.1 <= max_y {
        let next_pos1 = (sand_pos.0, sand_pos.1 + 1);
        let next_pos2 = (sand_pos.0 - 1, sand_pos.1 + 1);
        let next_pos3 = (sand_pos.0 + 1, sand_pos.1 + 1);

        if !map.contains(&next_pos1) {
            sand_pos = next_pos1;
        } else if !map.contains(&next_pos2) {
            sand_pos = next_pos2;
        } else if !map.contains(&next_pos3) {
            sand_pos = next_pos3;
        } else {
            break;
        }
    }

    if sand_pos.1 <= max_y {
        map.insert(sand_pos);
        true
    } else {
        false
    }
}

#[allow(dead_code)]
fn print_map(map: &HashSet<(i64, i64)>) {
    let ((min_x, min_y), (max_x, max_y)) = get_map_minmax(map);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part1(file_lines: &[String]) -> String {
    let mut map = parse_map(file_lines);
    let ((_, _), (_, max_y)) = get_map_minmax(&map);
    let mut sand_count = 0;
    while drop_sand(&mut map, max_y, 500, 0) {
        sand_count += 1;
    }
    sand_count.to_string()
}

fn drop_sand_p2(map: &mut HashSet<(i64, i64)>, max_y: i64, x: i64, y: i64) -> bool {
    let mut sand_pos = (x, y);
    if map.contains(&sand_pos) {
        return false;
    }

    while sand_pos.1 < max_y - 1 {
        let next_pos1 = (sand_pos.0, sand_pos.1 + 1);
        let next_pos2 = (sand_pos.0 - 1, sand_pos.1 + 1);
        let next_pos3 = (sand_pos.0 + 1, sand_pos.1 + 1);

        if !map.contains(&next_pos1) {
            sand_pos = next_pos1;
        } else if !map.contains(&next_pos2) {
            sand_pos = next_pos2;
        } else if !map.contains(&next_pos3) {
            sand_pos = next_pos3;
        } else {
            break;
        }
    }

    map.insert(sand_pos);
    true
}


fn part2(file_lines: &[String]) -> String {
    let mut map = parse_map(file_lines);
    let ((_, _), (_, max_y)) = get_map_minmax(&map);
    let mut sand_count = 0;
    while drop_sand_p2(&mut map, max_y + 2, 500, 0) {
        sand_count += 1;
    }
    sand_count.to_string()
}
