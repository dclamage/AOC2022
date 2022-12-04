use std::io::{self, Write};
use std::time::*;
use utility::*;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Parsing
    writeln!(stdout, "Parsing...").unwrap();
    let start_time = Instant::now();
    let file_lines = read_file_lines("day2/input.txt");
    //let file_lines = read_file_lines("day2/example_input.txt");
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

#[derive(Copy, Clone, PartialEq)]
enum RPSMove {
    Rock,
    Paper,
    Scissors,
}

fn beats(move_a: RPSMove, move_b: RPSMove) -> bool {
    matches!(
        (move_a, move_b),
        (RPSMove::Rock, RPSMove::Scissors)
            | (RPSMove::Paper, RPSMove::Rock)
            | (RPSMove::Scissors, RPSMove::Paper)
    )
}

fn move_score(m: RPSMove) -> u32 {
    match m {
        RPSMove::Rock => 1,
        RPSMove::Paper => 2,
        RPSMove::Scissors => 3,
    }
}

fn part1(file_lines: &[String]) -> String {
    let mut score = 0;
    for line in file_lines.iter() {
        let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let enemy_move = parts[0];
        let my_move = parts[1];
        let enemy_move = match enemy_move {
            "A" => RPSMove::Rock,
            "B" => RPSMove::Paper,
            "C" => RPSMove::Scissors,
            _ => panic!("Invalid enemy move"),
        };
        let my_move = match my_move {
            "X" => RPSMove::Rock,
            "Y" => RPSMove::Paper,
            "Z" => RPSMove::Scissors,
            _ => panic!("Invalid my move"),
        };
        score += move_score(my_move)
            + if enemy_move == my_move {
                3
            } else if beats(my_move, enemy_move) {
                6
            } else {
                0
            };
    }

    format!("{}", score)
}

fn winning_move(m: RPSMove) -> RPSMove {
    match m {
        RPSMove::Rock => RPSMove::Paper,
        RPSMove::Paper => RPSMove::Scissors,
        RPSMove::Scissors => RPSMove::Rock,
    }
}

fn losing_move(m: RPSMove) -> RPSMove {
    match m {
        RPSMove::Rock => RPSMove::Scissors,
        RPSMove::Paper => RPSMove::Rock,
        RPSMove::Scissors => RPSMove::Paper,
    }
}

fn part2(file_lines: &[String]) -> String {
    let mut score = 0;
    for line in file_lines.iter() {
        let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let enemy_move = parts[0];
        let my_move = parts[1];
        let enemy_move = match enemy_move {
            "A" => RPSMove::Rock,
            "B" => RPSMove::Paper,
            "C" => RPSMove::Scissors,
            _ => panic!("Invalid enemy move"),
        };
        let my_move = match my_move {
            "X" => losing_move(enemy_move),
            "Y" => enemy_move,
            "Z" => winning_move(enemy_move),
            _ => panic!("Invalid my move"),
        };
        score += move_score(my_move)
            + if enemy_move == my_move {
                3
            } else if beats(my_move, enemy_move) {
                6
            } else {
                0
            };
    }

    format!("{}", score)
}
