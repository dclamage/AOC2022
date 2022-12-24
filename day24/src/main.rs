use std::collections::{HashSet, VecDeque};
use std::io::{self, Write};
use std::time::*;
use utility::*;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Parsing
    writeln!(stdout, "Parsing...").unwrap();
    let start_time = Instant::now();
    let file_lines = read_file_lines("day24/input.txt");
    //let file_lines = read_file_lines("day24/example-input.txt");
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
enum Dir {
    N,
    S,
    E,
    W,
}

#[derive(Debug, Clone)]
struct Blizzards {
    locations: Vec<(i64, i64, Dir)>,
    locations_lookup: HashSet<(i64, i64)>,
    min: (i64, i64),
    max: (i64, i64),
}

impl Blizzards {
    fn from_lines(file_lines: &[String]) -> Self {
        let mut blizzards = Vec::new();
        let mut min = (0, 0);
        let mut max = (0, 0);
        for (r, line) in file_lines.iter().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                match ch {
                    '#' | '.' => {
                        min.0 = min.0.min(r as i64);
                        min.1 = min.1.min(c as i64);
                        max.0 = max.0.max(r as i64);
                        max.1 = max.1.max(c as i64);
                    }
                    '^' => blizzards.push((r as i64, c as i64, Dir::N)),
                    'v' => blizzards.push((r as i64, c as i64, Dir::S)),
                    '>' => blizzards.push((r as i64, c as i64, Dir::E)),
                    '<' => blizzards.push((r as i64, c as i64, Dir::W)),
                    _ => panic!("Unexpected character: {}", ch),
                }
            }
        }
        Self {
            locations_lookup: blizzards.iter().map(|(r, c, _)| (*r, *c)).collect(),
            locations: blizzards,
            min,
            max,
        }
    }

    fn next(&self) -> Self {
        let mut new_blizzards = Vec::with_capacity(self.locations.len());
        for (r, c, dir) in self.locations.iter() {
            let (r, c) = match dir {
                Dir::N => (r - 1, *c),
                Dir::S => (r + 1, *c),
                Dir::E => (*r, c + 1),
                Dir::W => (*r, c - 1),
            };
            if r == self.min.0 {
                new_blizzards.push((self.max.0 - 1, c, *dir));
            } else if r == self.max.0 {
                new_blizzards.push((self.min.0 + 1, c, *dir));
            } else if c == self.min.1 {
                new_blizzards.push((r, self.max.1 - 1, *dir));
            } else if c == self.max.1 {
                new_blizzards.push((r, self.min.1 + 1, *dir));
            } else {
                new_blizzards.push((r, c, *dir));
            }
        }
        Self {
            locations_lookup: new_blizzards.iter().map(|(r, c, _)| (*r, *c)).collect(),
            locations: new_blizzards,
            min: self.min,
            max: self.max,
        }
    }
}

impl PartialEq for Blizzards {
    fn eq(&self, other: &Self) -> bool {
        self.locations == other.locations
    }
}

impl Eq for Blizzards {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    minute: usize,
    position: (i64, i64),
}

impl State {
    #[allow(dead_code)]
    fn print(&self, blizzards: &Blizzards) {
        let entrance = (0, 1);
        let exit = (blizzards.max.0, blizzards.max.1 - 1);

        println!("Minute: {}", self.minute);
        for r in blizzards.min.0..=blizzards.max.0 {
            for c in blizzards.min.1..=blizzards.max.1 {
                let overlap_count = blizzards
                    .locations
                    .iter()
                    .filter(|(r2, c2, _)| r == *r2 && c == *c2)
                    .count();

                if self.position == (r, c) {
                    print!("E");
                } else if entrance == (r, c) {
                    print!("e");
                } else if exit == (r, c) {
                    print!("x");
                } else if overlap_count > 1 {
                    print!("{}", overlap_count);
                } else if blizzards.locations.contains(&(r, c, Dir::N)) {
                    print!("^");
                } else if blizzards.locations.contains(&(r, c, Dir::S)) {
                    print!("v");
                } else if blizzards.locations.contains(&(r, c, Dir::E)) {
                    print!(">");
                } else if blizzards.locations.contains(&(r, c, Dir::W)) {
                    print!("<");
                } else if r == blizzards.min.0
                    || r == blizzards.max.0
                    || c == blizzards.min.1
                    || c == blizzards.max.1
                {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn all_possible_blizzards(initial_blizzards: Blizzards) -> Vec<Blizzards> {
    let mut blizzard_state_per_minute = vec![initial_blizzards];
    loop {
        let next_blizzard_state = blizzard_state_per_minute.last().unwrap().next();
        if blizzard_state_per_minute[0] == next_blizzard_state {
            break;
        }
        blizzard_state_per_minute.push(next_blizzard_state);
    }
    blizzard_state_per_minute
}

fn find_shortest_path(
    initial_state: State,
    exit: (i64, i64),
    blizzard_state_per_minute: &[Blizzards],
) -> Option<usize> {
    let min = blizzard_state_per_minute[0].min;
    let max = blizzard_state_per_minute[0].max;

    // Search for the shortest path from start to end, dodging blizzards each minute
    let mut queue = VecDeque::new();
    queue.push_back(initial_state);
    let mut visited = HashSet::new();
    visited.insert(initial_state);
    while let Some(state) = queue.pop_front() {
        //state.print(&blizzard_state_per_minute[state.minute % blizzard_state_per_minute.len()]);
        assert!(state.position != exit);

        let next_blizzard_state =
            &blizzard_state_per_minute[(state.minute + 1) % blizzard_state_per_minute.len()];

        // Add do nothing state
        if !next_blizzard_state
            .locations_lookup
            .contains(&state.position)
        {
            let new_state = State {
                minute: state.minute + 1,
                position: state.position,
            };

            let equiv_state = State {
                minute: new_state.minute % blizzard_state_per_minute.len(),
                position: new_state.position,
            };

            if !visited.contains(&equiv_state) {
                queue.push_back(new_state);
                visited.insert(equiv_state);
            }
        }

        // Move in the four directions
        for dir in &[Dir::N, Dir::S, Dir::E, Dir::W] {
            let (r, c) = match dir {
                Dir::N => (state.position.0 - 1, state.position.1),
                Dir::S => (state.position.0 + 1, state.position.1),
                Dir::E => (state.position.0, state.position.1 + 1),
                Dir::W => (state.position.0, state.position.1 - 1),
            };
            if (r, c) == exit {
                return Some(state.minute + 1);
            }

            if r <= min.0
                || r >= max.0
                || c <= min.1
                || c >= max.1
                || next_blizzard_state.locations_lookup.contains(&(r, c))
            {
                continue;
            }
            let new_state = State {
                minute: state.minute + 1,
                position: (r, c),
            };

            let equiv_state = State {
                minute: new_state.minute % blizzard_state_per_minute.len(),
                position: new_state.position,
            };

            if !visited.contains(&equiv_state) {
                visited.insert(equiv_state);
                queue.push_back(new_state);
            }
        }
    }

    None
}

fn part1(file_lines: &[String]) -> String {
    let blizzard_state_per_minute = all_possible_blizzards(Blizzards::from_lines(file_lines));
    let max = blizzard_state_per_minute[0].max;
    let entrance = (0, 1);
    let exit = (max.0, max.1 - 1);

    let shortest_path = find_shortest_path(
        State {
            minute: 0,
            position: entrance,
        },
        exit,
        &blizzard_state_per_minute,
    )
    .unwrap();
    shortest_path.to_string()
}

fn part2(file_lines: &[String]) -> String {
    let blizzard_state_per_minute = all_possible_blizzards(Blizzards::from_lines(file_lines));
    let max = blizzard_state_per_minute[0].max;
    let entrance = (0, 1);
    let exit = (max.0, max.1 - 1);

    let shortest_path_a = find_shortest_path(
        State {
            minute: 0,
            position: entrance,
        },
        exit,
        &blizzard_state_per_minute,
    )
    .unwrap();

    let shortest_path_b = find_shortest_path(
        State {
            minute: shortest_path_a + 1,
            position: exit,
        },
        entrance,
        &blizzard_state_per_minute,
    ).unwrap();

    let shortest_path_c = find_shortest_path(
        State {
            minute: shortest_path_b + 1,
            position: entrance,
        },
        exit,
        &blizzard_state_per_minute,
    ).unwrap();

    shortest_path_c.to_string()
}
