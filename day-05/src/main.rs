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

fn solve_part2(input: &str) -> u64 {
    let parts: Vec<&str> = input.trim().split("\n\n").collect();
    if parts.len() < 2 {
        return 0;
    }

    let mut search_ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .filter_map(|line| {
            let mut s = line.split('-');
            let start = s.next()?.parse::<u64>().ok()?;
            let end = s.next()?.parse::<u64>().ok()?;
            Some((start, end))
        })
        .collect();

    if search_ranges.is_empty() {
        return 0;
    }

    // 1. Sort by start value
    search_ranges.sort_unstable_by_key(|r| r.0);

    // 2. Merge overlapping ranges
    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(search_ranges.len());
    let mut current = search_ranges[0];

    for next in search_ranges.into_iter().skip(1) {
        if next.0 <= current.1 {
            // Overlap: extend current end if next end is further
            current.1 = current.1.max(next.1);
        } else {
            // No overlap: push current and start new one
            merged.push(current);
            current = next;
        }
    }
    merged.push(current);

    // 3. Sum lengths
    merged.iter().map(|&(start, end)| end - start + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../example.txt");
        assert_eq!(solve_part1(input), 3);
    }

    #[test]
    fn test_part1_input_ne_0() {
        let input = include_str!("../input.txt");
        assert_ne!(solve_part1(input), 0);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../example.txt");
        assert_eq!(solve_part2(input), 14);
    }
}
