fn main() {
    let input = include_str!("../input.txt");
    let part1 = solve_part1(input);
    println!("Part 1: {}", part1);
    let part2 = solve_part2(input);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return 0;
            }

            // Find the maximum two-digit number we can form from any two batteries
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

            if digits.len() < 2 {
                return 0;
            }

            let mut max_joltage = 0;
            // Try all pairs of positions (i, j) where i < j
            for i in 0..digits.len() {
                for j in (i + 1)..digits.len() {
                    let joltage = digits[i] * 10 + digits[j];
                    max_joltage = max_joltage.max(joltage);
                }
            }

            max_joltage
        })
        .sum()
}

fn max_subsequence_joltage(line: &str, keep: usize) -> u64 {
    let digits: Vec<u8> = line
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect();

    if digits.len() < keep {
        return 0;
    }

    let mut stack = Vec::with_capacity(digits.len());
    let mut to_remove = digits.len() - keep;

    for &digit in &digits {
        while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < digit {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(digit);
    }

    while to_remove > 0 {
        stack.pop();
        to_remove -= 1;
    }

    stack.truncate(keep);

    stack
        .into_iter()
        .fold(0u64, |acc, digit| acc * 10 + digit as u64)
}

fn solve_part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return 0;
            }
            max_subsequence_joltage(trimmed, 12)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_from_problem() {
        let input = "\
987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(solve_part1(input), 357);
    }

    #[test]
    fn test_single_bank_987654321111111() {
        let input = "987654321111111";
        // The largest two-digit number is 98 (first two digits)
        assert_eq!(solve_part1(input), 98);
    }

    #[test]
    fn test_single_bank_811111111111119() {
        let input = "811111111111119";
        // The largest is 89 (digits 8 and 9)
        assert_eq!(solve_part1(input), 89);
    }

    #[test]
    fn test_single_bank_234234234234278() {
        let input = "234234234234278";
        // The largest is 78 (last two digits)
        assert_eq!(solve_part1(input), 78);
    }

    #[test]
    fn test_single_bank_818181911112111() {
        let input = "818181911112111";
        // The largest is 92 (digits 9 and 2)
        assert_eq!(solve_part1(input), 92);
    }

    #[test]
    fn test_single_bank_all_same_digit() {
        let input = "111111";
        // All pairs give 11
        assert_eq!(solve_part1(input), 11);
    }

    #[test]
    fn test_single_bank_two_digits() {
        let input = "12";
        assert_eq!(solve_part1(input), 12);
    }

    #[test]
    fn test_single_bank_descending() {
        let input = "987654321";
        // Largest is 98
        assert_eq!(solve_part1(input), 98);
    }

    #[test]
    fn test_single_bank_ascending() {
        let input = "123456789";
        // Largest is 89
        assert_eq!(solve_part1(input), 89);
    }

    #[test]
    fn test_single_bank_with_9_in_middle() {
        let input = "123945678";
        // Largest is 98 (9 and 8)
        assert_eq!(solve_part1(input), 98);
    }

    #[test]
    fn test_single_bank_999() {
        let input = "999";
        // All pairs give 99
        assert_eq!(solve_part1(input), 99);
    }

    #[test]
    fn test_single_bank_199() {
        let input = "199";
        // Pairs: 19, 19, 99 -> max is 99
        assert_eq!(solve_part1(input), 99);
    }

    #[test]
    fn test_single_bank_919() {
        let input = "919";
        // Pairs: 91, 99, 19 -> max is 99
        assert_eq!(solve_part1(input), 99);
    }

    #[test]
    fn test_single_bank_991() {
        let input = "991";
        // Pairs: 99, 91, 91 -> max is 99
        assert_eq!(solve_part1(input), 99);
    }

    #[test]
    fn test_single_bank_123() {
        let input = "123";
        // Pairs: 12, 13, 23 -> max is 23
        assert_eq!(solve_part1(input), 23);
    }

    #[test]
    fn test_single_bank_321() {
        let input = "321";
        // Pairs: 32, 31, 21 -> max is 32
        assert_eq!(solve_part1(input), 32);
    }

    #[test]
    fn test_single_bank_5678() {
        let input = "5678";
        // Pairs: 56, 57, 58, 67, 68, 78 -> max is 78
        assert_eq!(solve_part1(input), 78);
    }

    #[test]
    fn test_single_bank_8765() {
        let input = "8765";
        // Pairs: 87, 86, 85, 76, 75, 65 -> max is 87
        assert_eq!(solve_part1(input), 87);
    }

    #[test]
    fn test_empty_line() {
        let input = "";
        assert_eq!(solve_part1(input), 0);
    }

    #[test]
    fn test_single_digit() {
        let input = "5";
        // Need at least 2 digits
        assert_eq!(solve_part1(input), 0);
    }

    #[test]
    fn test_multiple_banks_simple() {
        let input = "\
12
34
56";
        // 12 + 34 + 56 = 102
        assert_eq!(solve_part1(input), 102);
    }

    #[test]
    fn test_multiple_banks_with_empty_lines() {
        let input = "\
12

34

56";
        // Empty lines should be ignored
        assert_eq!(solve_part1(input), 102);
    }

    #[test]
    fn test_bank_with_whitespace() {
        let input = "  987654321111111  ";
        // Should trim and work correctly
        assert_eq!(solve_part1(input), 98);
    }

    #[test]
    fn test_single_bank_1234567890() {
        let input = "1234567890";
        // Note: 0 is not a valid joltage (1-9), but if it appears, we should handle it
        // Actually, wait - the problem says joltage is 1-9, so maybe 0 shouldn't appear?
        // But let's test it anyway to be safe
        // Pairs with 0: 10, 20, 30, 40, 50, 60, 70, 80, 90
        // Pairs without 0: 12, 13, ..., 89
        // Max is 90
        assert_eq!(solve_part1(input), 90);
    }

    #[test]
    fn test_single_bank_9876543210() {
        let input = "9876543210";
        // Max should be 98 (9 and 8, ignoring 0)
        assert_eq!(solve_part1(input), 98);
    }

    #[test]
    fn test_single_bank_9012345678() {
        let input = "9012345678";
        // Max should be 98 (9 and 8), not 90
        assert_eq!(solve_part1(input), 98);
    }

    #[test]
    fn test_single_bank_123456789() {
        let input = "123456789";
        // All pairs, max is 89
        assert_eq!(solve_part1(input), 89);
    }

    #[test]
    fn test_single_bank_987654321() {
        let input = "987654321";
        // All pairs, max is 98
        assert_eq!(solve_part1(input), 98);
    }

    #[test]
    fn test_single_bank_1122334455() {
        let input = "1122334455";
        // Pairs: 11, 12, 13, 14, 15, 22, 23, 24, 25, 33, 34, 35, 44, 45, 55
        // Max is 55
        assert_eq!(solve_part1(input), 55);
    }

    #[test]
    fn test_single_bank_9988776655() {
        let input = "9988776655";
        // Max is 99
        assert_eq!(solve_part1(input), 99);
    }

    #[test]
    fn test_max_subsequence_joltage_example_inputs() {
        assert_eq!(max_subsequence_joltage("987654321111111", 12), 987654321111);
        assert_eq!(max_subsequence_joltage("811111111111119", 12), 811111111119);
        assert_eq!(max_subsequence_joltage("234234234234278", 12), 434234234278);
        assert_eq!(max_subsequence_joltage("818181911112111", 12), 888911112111);
    }

    #[test]
    fn test_max_subsequence_joltage_insufficient_digits() {
        assert_eq!(max_subsequence_joltage("1234567", 12), 0);
    }

    #[test]
    fn test_solve_part2_example_total() {
        let input = "\
987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(solve_part2(input), 3_121_910_778_619);
    }
}
