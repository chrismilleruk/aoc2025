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

    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|l| {
            let mut chars: Vec<char> = l.chars().collect();
            while chars.len() < max_len {
                chars.push(' ');
            }
            chars
        })
        .collect();

    let mut sep_cols = Vec::new();
    for x in 0..max_len {
        let mut all_spaces = true;
        for y in 0..grid.len() {
            if grid[y][x] != ' ' {
                all_spaces = false;
                break;
            }
        }
        if all_spaces {
            sep_cols.push(x);
        }
    }

    let mut problems = Vec::new();
    let mut start_x = 0;
    for &sep in &sep_cols {
        if sep > start_x {
            problems.push((start_x, sep));
        }
        start_x = sep + 1;
    }
    if start_x < max_len {
        problems.push((start_x, max_len));
    }

    let mut total = 0;
    for (s, e) in problems {
        let mut nums = Vec::new();
        let mut op = ' ';
        for y in 0..grid.len() {
            let row_part: String = grid[y][s..e].iter().collect();
            let trimmed = row_part.trim();
            if trimmed.is_empty() {
                continue;
            }
            if trimmed == "+" || trimmed == "*" {
                op = trimmed.chars().next().unwrap();
            } else if let Ok(n) = trimmed.parse::<u128>() {
                nums.push(n);
            }
        }

        if nums.is_empty() {
            continue;
        }

        let res = match op {
            '+' => nums.iter().sum::<u128>(),
            '*' => nums.iter().product::<u128>(),
            _ => 0,
        };
        total += res;
    }

    total
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
