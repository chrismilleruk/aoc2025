use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt");

    let start = Instant::now();
    let result = solve_part1(input);
    let duration = start.elapsed();

    println!("Part 1 result: {} ({:?})", result, duration);
}

fn solve_part1(input: &str) -> usize {
    let parts: Vec<&str> = input.trim().split("\n\n").collect();
    if parts.len() < 2 {
        return 0;
    }

    let search_ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .filter_map(|line| {
            let mut s = line.split('-');
            let start = s.next()?.parse::<u64>().ok()?;
            let end = s.next()?.parse::<u64>().ok()?;
            Some((start, end))
        })
        .collect();

    let numbers: Vec<u64> = parts[1]
        .lines()
        .filter_map(|line| line.trim().parse::<u64>().ok())
        .collect();

    numbers
        .iter()
        .filter(|&&n| {
            search_ranges
                .iter()
                .any(|&(start, end)| n >= start && n <= end)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = include_str!("../example.txt");
        assert_eq!(solve_part1(input), 3);
    }

    #[test]
    fn test_input_ne_0() {
        let input = include_str!("../input.txt");
        assert_ne!(solve_part1(input), 0);
    }
}
