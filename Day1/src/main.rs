use utility::*;

fn main() {
    // Parsing
    let file_lines = read_file_lines("day1/input.txt");
    let mut elves_vec = Vec::new();
    let mut cur_total = 0;
    for line in file_lines {
        let line = line.trim();
        if line.is_empty() {
            elves_vec.push(cur_total);
            cur_total = 0;
        } else {
            // Parse the line as an integer
            let line_int = line.parse::<i64>().unwrap();
            cur_total += line_int;
        }
    }

    // Part 1
    let max_total = elves_vec.iter().max().unwrap();
    println!("PART 1: Max total: {}", max_total);

    // Part 2
    elves_vec.sort_by(|a, b| b.cmp(a));
    let top_three = elves_vec[0..3].iter().sum::<i64>();
    println!("PART 2: Top three: {}", top_three);

}
