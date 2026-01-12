use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt");

    let start = Instant::now();
    let result = solve_part1(input);
    let duration = start.elapsed();
    println!("Part 1 result: {} ({:?})", result, duration);

    let start = Instant::now();
    let result = solve_part2(input);
    let duration = start.elapsed();
    println!("Part 2 result: {} ({:?})", result, duration);
}

fn solve_part1(input: &str) -> usize {
    0
}

fn solve_part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../example.txt");
        assert_eq!(solve_part1(input), 4277556);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../example.txt");
        assert_eq!(solve_part2(input), 0);
    }
}
