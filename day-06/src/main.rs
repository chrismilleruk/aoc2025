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

fn solve_part1(input: &str) -> u128 {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    if lines.is_empty() {
        return 0;
    }

    let width = lines[0].len();
    for line in &lines {
        assert_eq!(
            line.len(),
            width,
            "All input lines must have the same length"
        );
    }

    let mut total = 0;
    let mut block_start = None;

    for x in 0..=width {
        // Treat the end of the line as a gap
        let is_gap = if x == width {
            true
        } else {
            lines.iter().all(|line| line.as_bytes()[x] == b' ')
        };

        match (is_gap, block_start) {
            // Gap found while in a block: process the completed block
            (true, Some(start_idx)) => {
                total += process_block(&lines, start_idx, x);
                block_start = None;
            }
            // Non-gap character found while not in a block: start a new block
            (false, None) => {
                block_start = Some(x);
            }
            // Subsequent gap characters or non-gap characters without a transition: do nothing
            _ => {}
        }
    }

    total
}

fn process_block(lines: &[&str], start: usize, end: usize) -> u128 {
    let mut nums = Vec::new();
    let mut op = None;

    for line in lines {
        let chunk = line[start..end].trim();
        if chunk.is_empty() {
            continue;
        }

        if chunk == "+" || chunk == "*" {
            op = Some(chunk.chars().next().unwrap());
        } else if let Ok(n) = chunk.parse::<u128>() {
            nums.push(n);
        }
    }

    match op {
        Some('+') => nums.iter().sum(),
        Some('*') => nums.iter().product(),
        _ => 0,
    }
}

fn solve_part2(_input: &str) -> u128 {
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
