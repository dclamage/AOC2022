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
    let file_lines = read_file_lines("day12/input.txt");
    //let file_lines = read_file_lines("day12/example-input.txt");
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

struct ParsedInput {
    nodes: Vec<Vec<(usize, i64)>>,
    start: usize,
    end: usize,
    valid_starts: Vec<usize>,
}

impl ParsedInput {
    fn from_lines(file_lines: &[String]) -> ParsedInput {
        // Create easy lookup for heights
        let map_heights: Vec<Vec<usize>> = file_lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        if c == 'S' {
                            0
                        } else if c == 'E' {
                            'z' as usize - 'a' as usize
                        } else {
                            c as usize - 'a' as usize
                        }
                    })
                    .collect()
            })
            .collect();

        let height = map_heights.len();
        let width = map_heights[0].len();

        // Find the start and end locations
        let mut start = 0;
        let mut end = 0;
        let mut valid_starts = Vec::new();
        for (i, line) in file_lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = i * width + j;
                    valid_starts.push(start);
                } else if c == 'E' {
                    end = i * width + j;
                } else if c == 'a' {
                    valid_starts.push(i * width + j);
                }
            }
        }

        // Create nodes out of the map
        let mut nodes = Vec::new();
        for i in 0..height {
            for j in 0..width {
                let cur_height = map_heights[i][j];
                let mut node: Vec<(usize, i64)> = Vec::new();
                if i > 0 && map_heights[i - 1][j] <= cur_height + 1 {
                    node.push(((i - 1) * width + j, 1));
                }
                if i < height - 1 && map_heights[i + 1][j] <= cur_height + 1 {
                    node.push(((i + 1) * width + j, 1));
                }
                if j > 0 && map_heights[i][j - 1] <= cur_height + 1 {
                    node.push((i * width + j - 1, 1));
                }
                if j < width - 1 && map_heights[i][j + 1] <= cur_height + 1 {
                    node.push((i * width + j + 1, 1));
                }
                nodes.push(node);
            }
        }

        ParsedInput {
            nodes,
            start,
            end,
            valid_starts,
        }
    }
}

fn part1(file_lines: &[String]) -> String {
    let input = ParsedInput::from_lines(file_lines);
    let (dist, _) = dijkstra(&input.nodes, input.start, input.end);

    dist.to_string()
}

fn part2(file_lines: &[String]) -> String {
    let input = ParsedInput::from_lines(file_lines);
    let dist = dijkstra_multi_start(&input.nodes, &input.valid_starts, input.end);

    dist.to_string()
}

pub fn dijkstra_multi_start(graph: &Vec<Vec<(usize, i64)>>, starts: &[usize], end: usize) -> i64 {
    let mut dist = vec![std::i64::MAX; graph.len()];
    let mut prev = vec![0; graph.len()];
    let mut pq = std::collections::BinaryHeap::new();
    for start in starts.iter() {
        dist[*start] = 0;
        pq.push(std::cmp::Reverse((0, *start)));
    }
    while let Some(std::cmp::Reverse((d_u, u))) = pq.pop() {
        if d_u > dist[u] {
            continue;
        }
        for &(v, w) in &graph[u] {
            let d_v = d_u + w;
            if d_v < dist[v] {
                dist[v] = d_v;
                prev[v] = u;
                pq.push(std::cmp::Reverse((d_v, v)));
            }
        }
    }

    dist[end]
}
