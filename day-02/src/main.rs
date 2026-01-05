fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}

fn solve_part1(_input: &str) -> u32 {
    0
}

fn solve_part2(_input: &str) -> u32 {
    0
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
    fn test_part2() {
        let input = "";
        assert_eq!(solve_part2(input), 0);
    }
}
