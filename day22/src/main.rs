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
    let file_lines = read_file_lines("day22/input.txt");
    //let file_lines = read_file_lines("day22/example-input.txt");
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
struct Cursor {
    pos: (i32, i32),
    dir: (i32, i32),
}

impl Cursor {
    fn password(&self) -> i64 {
        let facing = match self.dir {
            (0, 1) => 0,
            (1, 0) => 1,
            (0, -1) => 2,
            (-1, 0) => 3,
            _ => panic!("Invalid direction"),
        };
        (self.pos.0 + 1) as i64 * 1000 + (self.pos.1 + 1) as i64 * 4 + facing
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Open,
    Wall,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    M(i32),
    R,
    L,
}

impl Instruction {
    fn move_cursor(cursor: Cursor, tiles: &HashMap<(i32, i32), Tile>, part1: bool) -> Cursor {
        let mut new_cursor = cursor;
        new_cursor.pos.0 += cursor.dir.0;
        new_cursor.pos.1 += cursor.dir.1;

        match tiles.get(&new_cursor.pos) {
            Some(Tile::Wall) => new_cursor = cursor,
            Some(Tile::Open) => (),
            _ => {
                if part1 {
                    match cursor.dir {
                        (0, 1) => {
                            new_cursor.pos.1 = tiles
                                .iter()
                                .map(|(key, _)| *key)
                                .filter(|(x, _)| *x == cursor.pos.0)
                                .map(|(_, y)| y)
                                .min()
                                .unwrap()
                        }
                        (1, 0) => {
                            new_cursor.pos.0 = tiles
                                .iter()
                                .map(|(key, _)| *key)
                                .filter(|(_, y)| *y == cursor.pos.1)
                                .map(|(x, _)| x)
                                .min()
                                .unwrap()
                        }
                        (0, -1) => {
                            new_cursor.pos.1 = tiles
                                .iter()
                                .map(|(key, _)| *key)
                                .filter(|(x, _)| *x == cursor.pos.0)
                                .map(|(_, y)| y)
                                .max()
                                .unwrap()
                        }
                        (-1, 0) => {
                            new_cursor.pos.0 = tiles
                                .iter()
                                .map(|(key, _)| *key)
                                .filter(|(_, y)| *y == cursor.pos.1)
                                .map(|(x, _)| x)
                                .max()
                                .unwrap()
                        }
                        _ => panic!("Unexpected direction: {:?}", cursor.dir),
                    }
                } else {
                    new_cursor = Self::move_cube_edge(cursor);
                }

                if tiles[&new_cursor.pos] == Tile::Wall {
                    new_cursor = cursor;
                }
            }
        }

        new_cursor
    }

    fn move_cube_edge(cursor: Cursor) -> Cursor {
        // This is specific to my input, which looks like this:
        //  12
        //  3
        // 45
        // 6
        const FACE_SIZE: i32 = 50;
        let current_face = match (cursor.pos.0 / FACE_SIZE, cursor.pos.1 / FACE_SIZE) {
            (0, 1) => 1,
            (0, 2) => 2,
            (1, 1) => 3,
            (2, 0) => 4,
            (2, 1) => 5,
            (3, 0) => 6,
            _ => panic!("Unexpected position: {:?}", cursor.pos),
        };

        let mut new_cursor = cursor;
        new_cursor.pos.0 += cursor.dir.0;
        new_cursor.pos.1 += cursor.dir.1;
        match current_face {
            1 => {
                match cursor.dir {
                    (0, 1) => {
                        // Do nothing
                    }
                    (1, 0) => {
                        // Do nothing
                    }
                    (0, -1) => {
                        new_cursor.dir = (0, 1);
                        new_cursor.pos.0 = FACE_SIZE * 3 - cursor.pos.0 - 1;
                        new_cursor.pos.1 = 0;
                    }
                    (-1, 0) => {
                        new_cursor.dir = (0, 1);
                        new_cursor.pos.0 = cursor.pos.1 + FACE_SIZE * 2;
                        new_cursor.pos.1 = 0;
                    }
                    _ => panic!("Unexpected direction: {:?}", cursor.dir),
                }
            }
            2 => {
                match cursor.dir {
                    (0, 1) => {
                        new_cursor.dir = (0, -1);
                        new_cursor.pos.0 = FACE_SIZE * 3 - cursor.pos.0 - 1;
                        new_cursor.pos.1 = cursor.pos.1 - FACE_SIZE;
                    }
                    (1, 0) => {
                        new_cursor.dir = (0, -1);
                        new_cursor.pos.0 = cursor.pos.1 - FACE_SIZE;
                        new_cursor.pos.1 = FACE_SIZE * 2 - 1;
                    }
                    (0, -1) => {
                        // Do nothing
                    }
                    (-1, 0) => {
                        new_cursor.dir = (-1, 0);
                        new_cursor.pos.0 = FACE_SIZE * 4 - 1;
                        new_cursor.pos.1 = cursor.pos.1 - FACE_SIZE * 2;
                    }
                    _ => panic!("Unexpected direction: {:?}", cursor.dir),
                }
            }
            3 => {
                match cursor.dir {
                    (0, 1) => {
                        new_cursor.dir = (-1, 0);
                        new_cursor.pos.0 = FACE_SIZE - 1;
                        new_cursor.pos.1 = cursor.pos.0 + FACE_SIZE;
                    }
                    (1, 0) => {
                        // Do nothing
                    }
                    (0, -1) => {
                        new_cursor.dir = (1, 0);
                        new_cursor.pos.0 = FACE_SIZE * 2;
                        new_cursor.pos.1 = cursor.pos.0 - FACE_SIZE;
                    }
                    (-1, 0) => {
                        // Do nothing
                    }
                    _ => panic!("Unexpected direction: {:?}", cursor.dir),
                }
            }
            4 => {
                match cursor.dir {
                    (0, 1) => {
                        // Do nothing
                    }
                    (1, 0) => {
                        // Do nothing
                    }
                    (0, -1) => {
                        new_cursor.dir = (0, 1);
                        new_cursor.pos.0 = 3 * FACE_SIZE - cursor.pos.0 - 1;
                        new_cursor.pos.1 = FACE_SIZE;
                    }
                    (-1, 0) => {
                        new_cursor.dir = (0, 1);
                        new_cursor.pos.0 = cursor.pos.1 + FACE_SIZE;
                        new_cursor.pos.1 = FACE_SIZE;
                    }
                    _ => panic!("Unexpected direction: {:?}", cursor.dir),
                }
            }
            5 => {
                match cursor.dir {
                    (0, 1) => {
                        new_cursor.dir = (0, -1);
                        new_cursor.pos.0 = 3 * FACE_SIZE - cursor.pos.0 - 1;
                        new_cursor.pos.1 = FACE_SIZE * 3 - 1;
                    }
                    (1, 0) => {
                        new_cursor.dir = (0, -1);
                        new_cursor.pos.0 = cursor.pos.1 + FACE_SIZE * 2;
                        new_cursor.pos.1 = FACE_SIZE - 1;
                    }
                    (0, -1) => {
                        // Do nothing
                    }
                    (-1, 0) => {
                        // Do nothing
                    }
                    _ => panic!("Unexpected direction: {:?}", cursor.dir),
                }
            }
            6 => {
                match cursor.dir {
                    (0, 1) => {
                        new_cursor.dir = (-1, 0);
                        new_cursor.pos.0 = FACE_SIZE * 3 - 1;
                        new_cursor.pos.1 = cursor.pos.0 - 2 * FACE_SIZE;
                    }
                    (1, 0) => {
                        new_cursor.dir = (1, 0);
                        new_cursor.pos.0 = 0;
                        new_cursor.pos.1 = cursor.pos.1 + FACE_SIZE * 2;
                    }
                    (0, -1) => {
                        new_cursor.dir = (1, 0);
                        new_cursor.pos.0 = 0;
                        new_cursor.pos.1 = cursor.pos.0 - FACE_SIZE * 2;
                    }
                    (-1, 0) => {
                        // Do nothing
                    }
                    _ => panic!("Unexpected direction: {:?}", cursor.dir),
                }
            }
            _ => panic!("Unexpected face: {}", current_face),
        }

        new_cursor
    }

    fn execute(&self, cursor: &Cursor, tiles: &HashMap<(i32, i32), Tile>, part1: bool) -> Cursor {
        match self {
            Instruction::M(dist) => {
                let mut new_cursor = *cursor;
                for _ in 0..*dist {
                    new_cursor = Self::move_cursor(new_cursor, tiles, part1);
                }
                new_cursor
            }
            Instruction::R => {
                // Rotate clockwise
                let new_dir = match cursor.dir {
                    (0, 1) => (1, 0),   // > -> v
                    (1, 0) => (0, -1),  // v -> <
                    (0, -1) => (-1, 0), // < -> ^
                    (-1, 0) => (0, 1),  // ^ -> >
                    _ => panic!("Unexpected direction: {:?}", cursor.dir),
                };
                Cursor {
                    pos: cursor.pos,
                    dir: new_dir,
                }
            }
            Instruction::L => {
                // Rotate counter-clockwise
                let new_dir = match cursor.dir {
                    (0, 1) => (-1, 0),  // > -> ^
                    (1, 0) => (0, 1),   // v -> >
                    (0, -1) => (1, 0),  // < -> v
                    (-1, 0) => (0, -1), // ^ -> <
                    _ => panic!("Unexpected direction: {:?}", cursor.dir),
                };
                Cursor {
                    pos: cursor.pos,
                    dir: new_dir,
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ParsedData {
    tiles: HashMap<(i32, i32), Tile>,
    instructions: Vec<Instruction>,
    start_pos: (i32, i32),
}

fn parse_lines(file_lines: &[String]) -> ParsedData {
    let mut tiles = HashMap::new();
    let mut start_pos = (-1, -1);
    for (row, line) in file_lines.iter().enumerate().take(file_lines.len() - 2) {
        for (col, cur_char) in line.chars().enumerate() {
            let tile = match cur_char {
                '#' => Tile::Wall,
                '.' => Tile::Open,
                ' ' => continue,
                _ => panic!("Unexpected character: {}", cur_char),
            };

            if start_pos == (-1, -1) {
                start_pos = (row as i32, col as i32);
            }

            tiles.insert((row as i32, col as i32), tile);
        }
    }

    let mut instructions = Vec::new();
    let instructions_line = file_lines[file_lines.len() - 1].as_bytes();
    let mut instruction_idx = 0;
    while instruction_idx < instructions_line.len() {
        let instruction = if instructions_line[instruction_idx] >= b'0'
            && instructions_line[instruction_idx] <= b'9'
        {
            let mut dist_str = String::new();
            while instruction_idx < instructions_line.len()
                && instructions_line[instruction_idx] >= b'0'
                && instructions_line[instruction_idx] <= b'9'
            {
                dist_str.push(instructions_line[instruction_idx] as char);
                instruction_idx += 1;
            }
            let dist = dist_str.parse().unwrap();
            Instruction::M(dist)
        } else {
            let instruction = match instructions_line[instruction_idx] as char {
                'R' => Instruction::R,
                'L' => Instruction::L,
                _ => panic!(
                    "Unexpected character: {}",
                    instructions_line[instruction_idx] as char
                ),
            };
            instruction_idx += 1;
            instruction
        };

        instructions.push(instruction);
    }

    ParsedData {
        tiles,
        instructions,
        start_pos,
    }
}

#[allow(dead_code)]
fn print_map(cursor: &Cursor, tiles: &HashMap<(i32, i32), Tile>) {
    let min_x = tiles
        .iter()
        .map(|(key, _)| *key)
        .map(|(x, _)| x)
        .min()
        .unwrap();
    let max_x = tiles
        .iter()
        .map(|(key, _)| *key)
        .map(|(x, _)| x)
        .max()
        .unwrap();
    let min_y = tiles
        .iter()
        .map(|(key, _)| *key)
        .map(|(_, y)| y)
        .min()
        .unwrap();
    let max_y = tiles
        .iter()
        .map(|(key, _)| *key)
        .map(|(_, y)| y)
        .max()
        .unwrap();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if (x, y) == cursor.pos {
                print!(
                    "{}",
                    match cursor.dir {
                        (0, 1) => '>',
                        (1, 0) => 'v',
                        (0, -1) => '<',
                        (-1, 0) => '^',
                        _ => panic!("Unexpected direction: {:?}", cursor.dir),
                    }
                );
            } else {
                print!(
                    "{}",
                    match tiles.get(&(x, y)) {
                        Some(Tile::Wall) => '#',
                        Some(Tile::Open) => '.',
                        _ => ' ',
                    }
                );
            }
        }
        println!();
    }
}

fn part1(file_lines: &[String]) -> String {
    let ParsedData {
        tiles,
        instructions,
        start_pos,
    } = parse_lines(file_lines);
    let mut cursor = Cursor {
        pos: start_pos,
        dir: (0, 1),
    };
    //print_map(&cursor, &tiles);

    for instruction in instructions {
        cursor = instruction.execute(&cursor, &tiles, true);
        //println!("{:?}", instruction);
        //print_map(&cursor, &tiles);
    }
    let password = cursor.password();

    password.to_string()
}

fn part2(file_lines: &[String]) -> String {
    let ParsedData {
        tiles,
        instructions,
        start_pos,
    } = parse_lines(file_lines);
    let mut cursor = Cursor {
        pos: start_pos,
        dir: (0, 1),
    };

    for instruction in instructions {
        cursor = instruction.execute(&cursor, &tiles, false);
    }
    let password = cursor.password();

    password.to_string()
}
