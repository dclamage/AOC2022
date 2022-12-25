use std::io::{self, Write};
use std::time::*;
use utility::*;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Parsing
    writeln!(stdout, "Parsing...").unwrap();
    let start_time = Instant::now();
    let file_lines = read_file_lines("day25/input.txt");
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

fn snafu_to_int(snafu: &str) -> i64 {
    let snafu = snafu.as_bytes();
    let mut multiplier = 1;
    let mut result = 0;
    for c in snafu.iter().rev() {
        result += match c {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'-' => -1,
            b'=' => -2,
            _ => panic!("Invalid snafu character: {}", *c as char),
        } * multiplier;
        multiplier *= 5;
    }
    result
}

fn int_to_snafu(mut n: i64) -> String {
    let mut result = Vec::new();
    while n != 0 {
        let remainder = n % 5;
        n /= 5;
        result.push(remainder);
    }

    loop {
        let incorrect_idx = result
            .iter()
            .enumerate()
            .find(|(_, &x)| x > 2)
            .map(|(i, _)| i);
        if let Some(idx) = incorrect_idx {
            result[idx] -= 5;
            if result.len() == idx + 1 {
                result.push(1);
            } else {
                result[idx + 1] += 1;
            }
        } else {
            break;
        }
    }

    result.iter().rev().map(|&x| match x {
        0 => '0',
        1 => '1',
        2 => '2',
        -1 => '-',
        -2 => '=',
        _ => panic!("Invalid snafu integer: {}", x),
    }).collect()
}

fn part1(file_lines: &[String]) -> String {
    let sum = file_lines
        .iter()
        .map(|line| snafu_to_int(line))
        .sum::<i64>();
    
    int_to_snafu(sum)
}

fn part2(_file_lines: &[String]) -> String {
    "Merry Christmas!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu() {
        assert_eq!(snafu_to_int("1=-0-2"), 1747);
        assert_eq!(snafu_to_int("12111"), 906);
        assert_eq!(snafu_to_int("2=0="), 198);
        assert_eq!(snafu_to_int("21"), 11);
        assert_eq!(snafu_to_int("2=01"), 201);
        assert_eq!(snafu_to_int("111"), 31);
        assert_eq!(snafu_to_int("20012"), 1257);
        assert_eq!(snafu_to_int("112"), 32);
        assert_eq!(snafu_to_int("1=-1="), 353);
        assert_eq!(snafu_to_int("1-12"), 107);
        assert_eq!(snafu_to_int("12"), 7);
        assert_eq!(snafu_to_int("1="), 3);
        assert_eq!(snafu_to_int("122"), 37);

        assert_eq!(int_to_snafu(1747), "1=-0-2");
        assert_eq!(int_to_snafu(906), "12111");
        assert_eq!(int_to_snafu(198), "2=0=");
        assert_eq!(int_to_snafu(11), "21");
        assert_eq!(int_to_snafu(201), "2=01");
        assert_eq!(int_to_snafu(31), "111");
        assert_eq!(int_to_snafu(1257), "20012");
        assert_eq!(int_to_snafu(32), "112");
        assert_eq!(int_to_snafu(353), "1=-1=");
        assert_eq!(int_to_snafu(107), "1-12");
        assert_eq!(int_to_snafu(7), "12");
        assert_eq!(int_to_snafu(3), "1=");
        assert_eq!(int_to_snafu(37), "122");

        for i in 0..100000 {
            assert_eq!(snafu_to_int(&int_to_snafu(i)), i);
        }
    }
}