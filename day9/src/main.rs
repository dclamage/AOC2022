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
    let file_lines = read_file_lines("day9/input.txt");
    //let file_lines = read_file_lines("day9/example-input.txt");
    //let file_lines = read_file_lines("day9/example-input2.txt");
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
enum Move {
    Down(usize),
    Up(usize),
    Left(usize),
    Right(usize),
}

impl Move {
    fn get_length(&self) -> usize {
        match self {
            Move::Down(d) => *d,
            Move::Up(d) => *d,
            Move::Left(d) => *d,
            Move::Right(d) => *d,
        }
    }
}

fn parse_moves(file_lines: &[String]) -> Vec<Move> {
    let mut moves = Vec::new();
    for line in file_lines {
        let (dir, dist) = line.split(' ').collect_tuple().unwrap();
        let dist: usize = dist.parse().unwrap();
        match dir.chars().next().unwrap() {
            'D' => moves.push(Move::Down(dist)),
            'U' => moves.push(Move::Up(dist)),
            'L' => moves.push(Move::Left(dist)),
            'R' => moves.push(Move::Right(dist)),
            _ => panic!("Invalid direction"),
        }
    }
    moves
}

fn adjust_tail(head_pos: (isize, isize), tail_pos: (isize, isize)) -> (isize, isize) {
    let (head_x, head_y) = head_pos;
    let (tail_x, tail_y) = tail_pos;
    let x_dist = (head_x - tail_x).abs();
    let y_dist = (head_y - tail_y).abs();
    if x_dist <= 1 && y_dist <= 1 {
        return tail_pos;
    }

    let x_move = (head_x - tail_x).signum();
    let y_move = (head_y - tail_y).signum();

    if x_dist == 0 {
        (tail_x, tail_y + y_move)
    } else if y_dist == 0 {
        (tail_x + x_move, tail_y)
    } else {
        (tail_x + x_move, tail_y + y_move)
    }
}

fn print_positions_visited(
    positions_visited: &HashSet<(isize, isize)>,
    knot_positions: &Vec<(isize, isize)>,
) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for (x, y) in positions_visited {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }
    for (x, y) in knot_positions {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut printed = false;
            for (knot_index, &knot_position) in knot_positions.iter().enumerate() {
                if (x, y) == knot_position {
                    if knot_index == 0 {
                        print!("H");
                    } else {
                        print!("{}", knot_index);
                    }
                    printed = true;
                    break;
                }
            }
            if !printed {
                if (x, y) == (0, 0) {
                    print!("s");
                } else if positions_visited.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }

    println!();
    println!();
}

fn part1(file_lines: &[String]) -> String {
    let moves = parse_moves(file_lines);
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    let mut positions_visited = HashSet::new();
    positions_visited.insert(tail_pos);

    for m in moves {
        match m {
            Move::Down(dist) => {
                for _ in 0..dist {
                    head_pos.1 += 1;
                    tail_pos = adjust_tail(head_pos, tail_pos);
                    positions_visited.insert(tail_pos);
                }
            }
            Move::Up(dist) => {
                for _ in 0..dist {
                    head_pos.1 -= 1;
                    tail_pos = adjust_tail(head_pos, tail_pos);
                    positions_visited.insert(tail_pos);
                }
            }
            Move::Left(dist) => {
                for _ in 0..dist {
                    head_pos.0 -= 1;
                    tail_pos = adjust_tail(head_pos, tail_pos);
                    positions_visited.insert(tail_pos);
                }
            }
            Move::Right(dist) => {
                for _ in 0..dist {
                    head_pos.0 += 1;
                    tail_pos = adjust_tail(head_pos, tail_pos);
                    positions_visited.insert(tail_pos);
                }
            }
        }
    }

    positions_visited.len().to_string()
}

fn part2(file_lines: &[String]) -> String {
    let moves = parse_moves(file_lines);
    let mut knot_pos_arr = vec![(0, 0); 10];
    let mut positions_visited = HashSet::new();
    positions_visited.insert(knot_pos_arr[9]);

    //print_positions_visited(&positions_visited, &knot_pos_arr);

    for m in moves {
        //println!("{:?}", m);
        for _ in 0..m.get_length() {
            match m {
                Move::Down(_) => {
                    knot_pos_arr[0].1 += 1;
                }
                Move::Up(_) => {
                    knot_pos_arr[0].1 -= 1;
                }
                Move::Left(_) => {
                    knot_pos_arr[0].0 -= 1;
                }
                Move::Right(_) => {
                    knot_pos_arr[0].0 += 1;
                }
            }

            for i in 1..10 {
                knot_pos_arr[i] = adjust_tail(knot_pos_arr[i - 1], knot_pos_arr[i]);
            }
            positions_visited.insert(knot_pos_arr[9]);
        }
        //print_positions_visited(&positions_visited, &knot_pos_arr);
    }

    positions_visited.len().to_string()
}
