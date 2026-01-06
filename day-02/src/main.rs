use std::{collections::HashSet, ops::Div};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> u64 {
    let mut sum_invalid = 0;
    for range in input.split(',').map(|s| s.trim()) {
        let (left_str, right_str) = range.split_once('-').unwrap();

        let left = left_str.parse::<u64>().expect("Invalid number");
        let right = right_str.parse::<u64>().expect("Invalid number");
        assert!(left <= right, "Invalid range");
        // assert!(left_str.len() == right_str.len(), "Imbalanced range");
        // assert!(left_str.len() % 2 == 0, "Imbalanced range");

        let in_range = |n: u64| n >= left && n <= right;

        // Get first few digits of left and right according to special logic:
        // eg. 1-15 >> 1-1, 95-105 >> 9-10, 890-1450 >> 8-14

        // 1=1, 2=1, 3=1, 4=2, 5=2, 6=3, 7=3, 8=4, 9=4, 10=5, ...
        let from = left_str[0..left_str.len().div(2).max(1)]
            .parse::<u64>()
            .expect("Invalid number");
        // 1=1, 2=1, 3=2, 4=2, 5=3, 6=3, 7=4, 8=4, 9=5, 10=5, ...
        let to = right_str[0..right_str.len().div_ceil(2)]
            .parse::<u64>()
            .expect("Invalid number");

        // println!("Range {}-{} - seeking {}-{}", left_str, right_str, from, to);

        for i in from..=to {
            let mul = 10_u64.pow(i.ilog10() + 1);
            let n = i * mul + i;
            if n < left {
                continue;
            }
            if n > right {
                break;
            }
            // print!("{} ", n);
            if in_range(n) {
                // print!("| {} ", n);
                sum_invalid += n;
            }
        }

        // println!("+ Range {} - {}", range, sum_invalid);
    }
    sum_invalid
}

fn solve_part2(input: &str) -> u64 {
    let mut invalid_numbers = HashSet::new();
    for range in input.split(',').map(|s| s.trim()) {
        let (left_str, right_str) = range.split_once('-').unwrap();

        let left = left_str.parse::<u64>().expect("Invalid number");
        let right = right_str.parse::<u64>().expect("Invalid number");
        assert!(left <= right, "Invalid range");

        // Determine min/max digit lengths of numbers in the range
        let len_min = left_str.len().min(right_str.len());
        let len_max = left_str.len().max(right_str.len());

        // For each length L from min to max
        for l in len_min..=len_max {
            // Find all divisors k of L where k >= 2 (number of repetitions)
            let repetitions: Vec<usize> = (2..=l).filter(|k| l % k == 0).collect();

            for k in repetitions {
                // Pattern length = L / k
                let pattern_len = l / k;

                // Calculate multiplier using geometric series formula:
                // multiplier = 1 + 10^pattern_len + 10^(2*pattern_len) + ... + 10^((k-1)*pattern_len)
                // This equals: (10^(k*pattern_len) - 1) / (10^pattern_len - 1)
                let base = 10_u64.pow(pattern_len as u32);
                let multiplier = if base == 1 {
                    k as u64
                } else {
                    (base.pow(k as u32) - 1) / (base - 1)
                };

                // Calculate valid pattern range
                // min_pattern = ceil(left / multiplier)
                let min_pattern = (left + multiplier - 1) / multiplier;
                // max_pattern = floor(right / multiplier)
                let max_pattern = right / multiplier;

                // Also need to ensure patterns have exactly pattern_len digits
                let min_pattern_digits = 10_u64.pow((pattern_len - 1).max(0) as u32);
                let max_pattern_digits = 10_u64.pow(pattern_len as u32) - 1;

                // Clamp pattern range to valid digit range
                let pattern_start = min_pattern.max(min_pattern_digits);
                let pattern_end = max_pattern.min(max_pattern_digits);

                if pattern_start > pattern_end {
                    continue;
                }

                // For each pattern in the valid range
                for pattern in pattern_start..=pattern_end {
                    let candidate = pattern * multiplier;

                    // Verify it's in [left, right] and has exactly L digits
                    if candidate < left || candidate > right {
                        continue;
                    }

                    // Check that the candidate has exactly L digits
                    let candidate_digits = candidate.ilog10() + 1;
                    if candidate_digits != l as u32 {
                        continue;
                    }

                    // Verify it's actually a repeated pattern
                    let pattern_str = pattern.to_string();
                    let expected = pattern_str.repeat(k);
                    let candidate_str = candidate.to_string();
                    if expected == candidate_str {
                        invalid_numbers.insert(candidate);
                    }
                }
            }
        }
    }
    invalid_numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
";
        // In the above example:

        // 11-22 has two invalid IDs, 11 and 22.
        // 95-115 has one invalid ID, 99.
        // 998-1012 has one invalid ID, 1010.
        // 1188511880-1188511890 has one invalid ID, 1188511885.
        // 222220-222224 has one invalid ID, 222222.
        // 1698522-1698528 contains no invalid IDs.
        // 446443-446449 has one invalid ID, 446446.
        // 38593856-38593862 has one invalid ID, 38593859.
        // The rest of the ranges contain no invalid IDs.
        // Adding up all the invalid IDs in this example produces 1227775554.

        assert_eq!(solve_part1(input), 1227775554);
    }

    #[test]
    fn test_part1_11_22() {
        let input = "11-22";
        assert_eq!(solve_part1(input), 33);
    }

    #[test]
    fn test_part1_95_115() {
        let input = "95-115";
        assert_eq!(solve_part1(input), 99);
    }

    #[test]
    fn test_part1_1_13() {
        let input = "1-13";
        assert_eq!(solve_part1(input), 11);
    }

    // 8989806846-8989985017
    #[test]
    fn test_part1_8989806846_8989985017() {
        let input = "8989806846-8989985017";
        assert_eq!(solve_part1(input), 8989889898);
    }

    #[test]
    fn test_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
";
        // From the same example as before:

        // 11-22 still has two invalid IDs, 11 and 22.
        // 95-115 now has two invalid IDs, 99 and 111.
        // 998-1012 now has two invalid IDs, 999 and 1010.
        // 1188511880-1188511890 still has one invalid ID, 1188511885.
        // 222220-222224 still has one invalid ID, 222222.
        // 1698522-1698528 still contains no invalid IDs.
        // 446443-446449 still has one invalid ID, 446446.
        // 38593856-38593862 still has one invalid ID, 38593859.
        // 565653-565659 now has one invalid ID, 565656.
        // 824824821-824824827 now has one invalid ID, 824824824.
        // 2121212118-2121212124 now has one invalid ID, 2121212121.
        // Adding up all the invalid IDs in this example produces 4174379265.

        assert_eq!(solve_part2(input), 4174379265);
    }
}
