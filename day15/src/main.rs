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
    let file_lines = read_file_lines("day15/input.txt");
    //let file_lines = read_file_lines("day15/example-input.txt");
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SensorBeaconPair {
    sensor: (i64, i64),
    beacon: (i64, i64),
    distance: i64,
}

impl SensorBeaconPair {
    fn new(sensor: (i64, i64), beacon: (i64, i64)) -> Self {
        let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        Self {
            sensor,
            beacon,
            distance,
        }
    }

    fn from_string(s: &str) -> Self {
        // The hackiest hack that ever did hack and it's not even fast or efficient
        let sections = s.split(": closest beacon is at ").collect_vec();
        let sensor_sections = sections[0].split(", ").collect_vec();
        let beacon_sections = sections[1].split(", ").collect_vec();
        let sensor_xstring = sensor_sections[0].split('=').collect_vec()[1];
        let sensor_ystring = sensor_sections[1].split('=').collect_vec()[1];
        let beacon_xstring = beacon_sections[0].split('=').collect_vec()[1];
        let beacon_ystring = beacon_sections[1].split('=').collect_vec()[1];
        let sensor = (
            sensor_xstring.parse::<i64>().unwrap(),
            sensor_ystring.parse::<i64>().unwrap(),
        );
        let beacon = (
            beacon_xstring.parse::<i64>().unwrap(),
            beacon_ystring.parse::<i64>().unwrap(),
        );
        Self::new(sensor, beacon)
        // But it works
    }

    fn within_sensor_range(&self, other: (i64, i64)) -> bool {
        let distance = (self.sensor.0 - other.0).abs() + (self.sensor.1 - other.1).abs();
        distance <= self.distance
    }

    fn can_contain_unseen_points(&self, min: (i64, i64), max: (i64, i64)) -> bool {
        let corners = [
            (min.0, min.1),
            (min.0, max.1),
            (max.0, min.1),
            (max.0, max.1),
        ];
        let largest_dist = corners
            .iter()
            .map(|corner| (corner.0 - self.sensor.0).abs() + (corner.1 - self.sensor.1).abs())
            .max()
            .unwrap();
        largest_dist > self.distance
    }
}

fn parse_map(file_lines: &[String]) -> Vec<SensorBeaconPair> {
    file_lines
        .iter()
        .map(|line| SensorBeaconPair::from_string(line))
        .collect_vec()
}

fn part1(file_lines: &[String]) -> String {
    let map = parse_map(file_lines);

    // Occupied positions are the ones where a beacon or sensor exists
    let occupied_positions: HashSet<(i64, i64)> = map
        .iter()
        .flat_map(|pair| [pair.sensor, pair.beacon])
        .collect();

    // Change to 10 for example input
    const LINE_Y: i64 = 2000000;
    let min_x = map
        .iter()
        .map(|pair| pair.sensor.0.min(pair.beacon.0))
        .min()
        .unwrap();
    let max_x = map
        .iter()
        .map(|pair| pair.sensor.0.max(pair.beacon.0))
        .max()
        .unwrap();
    let max_range = map.iter().map(|pair| pair.distance).max().unwrap();
    let start_x = min_x - max_range;
    let end_x = max_x + max_range;

    let mut num_points_in_range = 0;
    for x in start_x..=end_x {
        let position = (x, LINE_Y);
        if occupied_positions.contains(&position) {
            continue;
        }
        if map.iter().any(|pair| pair.within_sensor_range(position)) {
            num_points_in_range += 1;
        }
    }
    num_points_in_range.to_string()
}

fn find_unseen_point(
    map: &[SensorBeaconPair],
    min: (i64, i64),
    max: (i64, i64),
) -> Option<(i64, i64)> {
    let mut quadrant_stack = vec![(min, max)];

    while let Some((min, max)) = quadrant_stack.pop() {
        if min == max {
            if map.iter().all(|pair| !pair.within_sensor_range(min)) {
                return Some(min);
            }
        } else {
            let mid = ((min.0 + max.0) / 2, (min.1 + max.1) / 2);
            let quadrants = [
                (min, mid),
                ((mid.0 + 1, min.1), (max.0, mid.1)),
                ((min.0, mid.1 + 1), (mid.0, max.1)),
                ((mid.0 + 1, mid.1 + 1), max),
            ];
            for quadrant in quadrants.iter() {
                if quadrant.0.0 > quadrant.1.0 || quadrant.0.1 > quadrant.1.1 {
                    continue;
                }

                if map
                    .iter()
                    .all(|pair| pair.can_contain_unseen_points(quadrant.0, quadrant.1))
                {
                    quadrant_stack.push(*quadrant);
                }
            }
        }
    }

    None
}

fn part2(file_lines: &[String]) -> String {
    let map = parse_map(file_lines);

    const MIN_XY: i64 = 0;
    //const MAX_XY: i64 = 20;
    const MAX_XY: i64 = 4000000;
    let min = (MIN_XY, MIN_XY);
    let max = (MAX_XY, MAX_XY);
    let found_position = find_unseen_point(&map, min, max).unwrap();

    let tuning_freq = found_position.0 * MAX_XY + found_position.1;
    tuning_freq.to_string()
}
