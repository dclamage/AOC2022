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
    let file_lines = read_file_lines("day18/input.txt");
    //let file_lines = read_file_lines("day18/example-input.txt");
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
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    fn new(x: i64, y: i64, z: i64) -> Point3D {
        Point3D { x, y, z }
    }

    fn from_string(line: &str) -> Point3D {
        let segs = line.split(',').collect_vec();
        Point3D {
            x: segs[0].parse().unwrap(),
            y: segs[1].parse().unwrap(),
            z: segs[2].parse().unwrap(),
        }
    }

    fn adjacent(&self) -> Point3DAdjacentIterator {
        Point3DAdjacentIterator::new(*self)
    }
}

struct Point3DAdjacentIterator {
    point: Point3D,
    index: usize,
}

impl Point3DAdjacentIterator {
    fn new(point: Point3D) -> Point3DAdjacentIterator {
        Point3DAdjacentIterator { point, index: 0 }
    }
}

impl Iterator for Point3DAdjacentIterator {
    type Item = Point3D;

    fn next(&mut self) -> Option<Point3D> {
        let Point3DAdjacentIterator { point, index } = self;
        let Point3D { x, y, z } = point;

        let result = match index {
            0 => Some(Point3D::new(*x - 1, *y, *z)),
            1 => Some(Point3D::new(*x + 1, *y, *z)),
            2 => Some(Point3D::new(*x, *y - 1, *z)),
            3 => Some(Point3D::new(*x, *y + 1, *z)),
            4 => Some(Point3D::new(*x, *y, *z - 1)),
            5 => Some(Point3D::new(*x, *y, *z + 1)),
            _ => None,
        };

        *index += 1;
        result
    }
}

fn parse_input(file_lines: &[String]) -> HashSet<Point3D> {
    file_lines
        .iter()
        .map(|line| Point3D::from_string(line))
        .collect()
}

fn total_surface_area(cubes: &HashSet<Point3D>) -> i64 {
    let mut surface_area = 0;
    for &cube in cubes.iter() {
        for adjacent_cube in cube.adjacent() {
            if !cubes.contains(&adjacent_cube) {
                surface_area += 1;
            }
        }
    }

    surface_area
}

fn part1(file_lines: &[String]) -> String {
    let cubes = parse_input(file_lines);
    let surface_area = total_surface_area(&cubes);
    surface_area.to_string()
}

fn part2(file_lines: &[String]) -> String {
    let mut cubes = parse_input(file_lines);
    let initial_surface_area = total_surface_area(&cubes);

    // Take an exanded bounding box of the cube and flood-fill it with cubes
    // This will leave only internal gaps.
    let min_x = cubes.iter().map(|cube| cube.x).min().unwrap() - 2;
    let max_x = cubes.iter().map(|cube| cube.x).max().unwrap() + 2;
    let min_y = cubes.iter().map(|cube| cube.y).min().unwrap() - 2;
    let max_y = cubes.iter().map(|cube| cube.y).max().unwrap() + 2;
    let min_z = cubes.iter().map(|cube| cube.z).min().unwrap() - 2;
    let max_z = cubes.iter().map(|cube| cube.z).max().unwrap() + 2;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            cubes.insert(Point3D::new(x, y, min_z));
            cubes.insert(Point3D::new(x, y, max_z));
        }
    }
    for x in min_x..=max_x {
        for z in min_z..=max_z {
            cubes.insert(Point3D::new(x, min_y, z));
            cubes.insert(Point3D::new(x, max_y, z));
        }
    }
    for y in min_y..=max_y {
        for z in min_z..=max_z {
            cubes.insert(Point3D::new(min_x, y, z));
            cubes.insert(Point3D::new(max_x, y, z));
        }
    }

    let start = Point3D::new(min_x + 1, min_y + 1, min_z + 1);
    let mut queue = vec![start];
    while let Some(cube) = queue.pop() {
        if cubes.insert(cube) {
            for adjacent_cube in cube.adjacent() {
                if !cubes.contains(&adjacent_cube) {
                    queue.push(adjacent_cube);
                }
            }
        }
    }

    let expected_new_external_surface_area = 2 * (max_x - min_x + 1) * (max_y - min_y + 1)
        + 2 * (max_x - min_x + 1) * (max_z - min_z + 1)
        + 2 * (max_y - min_y + 1) * (max_z - min_z + 1);
    let internal_surface_area = total_surface_area(&cubes) - expected_new_external_surface_area;
    let external_surface_area = initial_surface_area - internal_surface_area;

    external_surface_area.to_string()
}
