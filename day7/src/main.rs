use itertools::Itertools;
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
    let file_lines = read_file_lines("day7/input.txt");
    //let file_lines = read_file_lines("day7/example-input.txt");
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

#[derive(Debug)]
enum LineContents {
    None,
    CD(String),
    LS,
    Dir(String),
    File(String, usize),
}

fn parse_line(line: &str) -> LineContents {
    if line.is_empty() {
        LineContents::None
    } else if line.as_bytes()[0] == b'$' {
        parse_command(&line[2..])
    } else {
        parse_output(line)
    }
}

fn parse_command(line: &str) -> LineContents {
    let mut parts = line.split_whitespace();
    let command = parts.next().unwrap();
    match command {
        "cd" => LineContents::CD(parts.next().unwrap().to_owned()),
        "ls" => LineContents::LS,
        _ => panic!("Unknown command: {}", command),
    }
}

fn parse_output(line: &str) -> LineContents {
    let mut parts = line.split_whitespace();
    let size = parts.next().unwrap();
    let name = parts.next().unwrap();
    if size == "dir" {
        LineContents::Dir(name.to_owned())
    } else {
        LineContents::File(name.to_owned(), size.parse().unwrap())
    }
}

#[derive(Debug)]
struct FileInfo {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct DirInfo {
    name: String,
    files: Vec<FileInfo>,
    dirs: Vec<String>,
}

impl DirInfo {
    fn new(name: String) -> DirInfo {
        DirInfo {
            name,
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }

    fn add_file(&mut self, file: FileInfo) {
        if !self.files.iter().any(|f| f.name == file.name) {
            self.files.push(file);
        }
    }

    fn add_dir(&mut self, dir: &str) {
        if !self.dirs.iter().any(|d| d == dir) {
            self.dirs.push(dir.to_owned());
        }
    }

    fn recursive_size(&self, folders: &HashMap<String, DirInfo>) -> usize {
        self.files.iter().map(|file| file.size).sum::<usize>()
            + self
                .dirs
                .iter()
                .map(|dir| folders.get(dir).unwrap().recursive_size(folders))
                .sum::<usize>()
    }
}

fn parse_folders(file_lines: &[String]) -> HashMap::<String, DirInfo> {
    let lines = file_lines.iter().map(|line| parse_line(line)).collect_vec();
    let mut folders = HashMap::<String, DirInfo>::new();
    // Push the root / folder
    folders.insert("/".to_owned(), DirInfo::new("/".to_owned()));
    let mut current_dir = "/".to_owned();
    for line in lines {
        match line {
            LineContents::None => (),
            LineContents::CD(dir) => {
                if dir == ".." {
                    // Go up a directory
                    let path = current_dir.split('/').collect_vec();
                    current_dir = path[..path.len() - 1].join("/");
                } else if dir.starts_with('/') {
                    // Go to a specific directory
                    current_dir = dir.clone();
                } else {
                    // Enter a sub-folder
                    if !current_dir.ends_with('/') {
                        current_dir += "/";
                    }
                    current_dir += &dir;
                }
            }
            LineContents::LS => {
                // Do nothing
            }
            LineContents::Dir(dir_name) => {
                let mut full_path = current_dir.clone();
                if !full_path.ends_with('/') {
                    full_path += "/";
                }
                full_path += &dir_name;

                folders
                    .get_mut(&current_dir)
                    .unwrap()
                    .add_dir(&full_path);
                if !folders.contains_key(&full_path) {
                    folders.insert(full_path.clone(), DirInfo::new(full_path.clone()));
                }
            }
            LineContents::File(file, size) => {
                folders
                    .get_mut(&current_dir)
                    .unwrap()
                    .add_file(FileInfo { name: file, size });
            }
        }
    }

    folders
}

fn part1(file_lines: &[String]) -> String {
    const SMALL_FOLDER_THRESHOLD: usize = 100000;

    let folders = parse_folders(file_lines);

    let folder_name_and_recursive_sizes = folders
        .values()
        .map(|dir| (dir.name.clone(), dir.recursive_size(&folders)))
        .collect_vec();

    let size_of_small_folders = folder_name_and_recursive_sizes
        .iter()
        .filter(|(_, size)| *size <= SMALL_FOLDER_THRESHOLD)
        .map(|(_, size)| size)
        .sum::<usize>();

    size_of_small_folders.to_string()
}

fn part2(file_lines: &[String]) -> String {
    const TOTAL_SPACE: usize = 70000000;
    const TOTAL_SPACE_NEEDED: usize = 30000000;

    let folders = parse_folders(file_lines);
    let space_used = folders.get("/").unwrap().recursive_size(&folders);
    let space_free = TOTAL_SPACE - space_used;
    let space_needed = TOTAL_SPACE_NEEDED - space_free;

    let potential_folders_to_delete = folders
        .values()
        .map(|dir| (dir.name.clone(), dir.recursive_size(&folders)))
        .filter(|(_, size)| *size >= space_needed)
        .collect_vec();

    let best_folder_to_delete = potential_folders_to_delete
        .iter()
        .min_by_key(|(_, size)| *size)
        .unwrap();

    format!("{}: {}", best_folder_to_delete.0, best_folder_to_delete.1)
}
