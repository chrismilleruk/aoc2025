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

fn solve_part2(input: &str) -> u128 {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    if lines.is_empty() {
        return 0;
    }

    let width = lines[0].len();
    let mut total = 0;
    let mut is_in_block = false;

    let mut current_op = None;
    let mut current_nums = Vec::new();
    let mut col_buffer = Vec::with_capacity(lines.len());

    for x in 0..=width {
        // Collect once into memory
        col_buffer.clear();
        if x < width {
            col_buffer.extend(lines.iter().map(|l| l.as_bytes()[x]));
        } else {
            col_buffer.resize(lines.len(), b' ');
        }
        let col_bytes = &col_buffer;

        let is_gap = col_bytes.iter().all(|&b| b == b' ');

        // Vertical parsing: collect digits in this column into one number
        let mut vertical_num = 0;
        let mut has_digit = false;
        for &b in col_bytes.iter().filter(|&&b| b.is_ascii_digit()) {
            vertical_num = vertical_num * 10 + (b - b'0') as u128;
            has_digit = true;
        }
        if has_digit {
            current_nums.push(vertical_num);
        }

        // Find operator if present in this column
        if let Some(&b) = col_bytes.iter().find(|&&b| b == b'+' || b == b'*') {
            current_op = Some(b as char);
        }

        match (is_gap, is_in_block) {
            (true, true) => {
                total += match current_op {
                    Some('+') => current_nums.iter().sum(),
                    Some('*') => current_nums.iter().product(),
                    _ => 0,
                };

                is_in_block = false;
                current_op = None;
                current_nums.clear();
            }
            (false, false) => {
                is_in_block = true;
            }
            _ => {}
        }
    }

    total
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
        assert_eq!(solve_part2(input), 3263827);
    }
}
