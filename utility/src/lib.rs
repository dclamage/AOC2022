use std::{fs::File, io::Read};

pub fn read_file(file: &str) -> String {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    contents
}

pub fn read_file_lines(file: &str) -> Vec<String> {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    contents.lines().map(|s| s.to_string()).collect()
}

pub fn read_file_tokens(file: &str) -> Vec<String> {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    contents.split_whitespace().map(|s| s.to_string()).collect()
}

// Useful math utility functions
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

pub fn lcm_many(numbers: &[i64]) -> i64 {
    let mut result = numbers[0];
    for i in 1..numbers.len() {
        result = lcm(result, numbers[i]);
    }
    result
}

pub fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

// Modular math
pub fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp = exp >> 1;
        base = (base * base) % modulus;
    }
    result
}

pub fn mod_inverse(a: i64, modulus: i64) -> i64 {
    let mut mn = (modulus, a);
    let mut xy = (0, 1);
    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }
    while xy.0 < 0 {
        xy.0 += modulus;
    }
    xy.0
}

pub fn crt(a: &[i64], m: &[i64]) -> i64 {
    let prod = m.iter().product::<i64>();
    let mut sum = 0;
    for (&a_i, &m_i) in a.iter().zip(m.iter()) {
        let p = prod / m_i;
        sum += a_i * mod_inverse(p, m_i) * p;
    }
    sum % prod
}

// Graph theory

// Dijkstra's algorithm
// Returns a vector of (distance, previous node) tuples
// The distance is std::i64::MAX if there is no path
// The graph is represented as an adjacency list
// Each element of the adjacency list is a tuple of (node, edge weight)
pub fn dijkstra(
    graph: &Vec<Vec<(usize, i64)>>,
    start: usize,
    end: usize,
) -> (i64, Vec<usize>) {
    let mut dist = vec![std::i64::MAX; graph.len()];
    let mut prev = vec![0; graph.len()];
    let mut pq = std::collections::BinaryHeap::new();
    dist[start] = 0;
    pq.push(std::cmp::Reverse((0, start)));
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
    let mut path = Vec::new();
    let mut u = end;
    while u != start {
        path.push(u);
        u = prev[u];
    }
    path.push(start);
    path.reverse();
    (dist[end], path)
}

// Tests
#[cfg(test)]
mod test {
    use super::*;

    // Test GCD and LCM
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(2, 10), 2);
        assert_eq!(gcd(10, 2), 2);
        assert_eq!(gcd(3, 10), 1);
        assert_eq!(gcd(10, 3), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(2, 10), 10);
        assert_eq!(lcm(10, 2), 10);
        assert_eq!(lcm(3, 10), 30);
        assert_eq!(lcm(10, 3), 30);
    }

    #[test]
    fn test_lcm_many() {
        assert_eq!(lcm_many(&[2, 10]), 10);
        assert_eq!(lcm_many(&[10, 2]), 10);
        assert_eq!(lcm_many(&[3, 10]), 30);
        assert_eq!(lcm_many(&[10, 3]), 30);
        assert_eq!(lcm_many(&[2, 3, 10]), 30);
        assert_eq!(lcm_many(&[10, 3, 2]), 30);
        assert_eq!(lcm_many(&[2, 3, 10, 11]), 330);
        assert_eq!(lcm_many(&[11, 10, 3, 2]), 330);
    }

    // Test primality
    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(12), false);
        assert_eq!(is_prime(13), true);
        assert_eq!(is_prime(14), false);
        assert_eq!(is_prime(15), false);
        assert_eq!(is_prime(16), false);
        assert_eq!(is_prime(17), true);
        assert_eq!(is_prime(18), false);
        assert_eq!(is_prime(19), true);
        assert_eq!(is_prime(20), false);
    }

    // Test mod operations
    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 3, 5), 3);
        assert_eq!(mod_pow(2, 4, 5), 1);
        assert_eq!(mod_pow(3, 3, 5), 2);
    }

    // Test Chinese Remainder Theorem
    #[test]
    fn test_crt() {
        let a = [2, 3, 2];
        let m = [3, 5, 7];
        assert_eq!(super::crt(&a, &m), 23);
    }

    // Test graph theory
    #[test]
    fn test_dijkstra() {
        let graph = vec![
            vec![(1, 7), (2, 9), (5, 14)],
            vec![(0, 7), (2, 10), (3, 15)],
            vec![(0, 9), (1, 10), (3, 11), (5, 2)],
            vec![(1, 15), (2, 11), (4, 6)],
            vec![(3, 6), (5, 9)],
            vec![(0, 14), (2, 2), (4, 9)],
        ];
        let (dist, path) = dijkstra(&graph, 0, 4);
        assert_eq!(dist, 20);
        assert_eq!(path, vec![0, 2, 5, 4]);
    }
}