fn main() {
    let input = include_str!("../input.txt");
    let part1 = solve_part1(input);
    println!("Part 1 - Password: {}", part1);
    let part2 = solve_part2(&input);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> u32 {
    let mut pos = 50;
    let mut count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse direction and amount
        let dir = &line[0..1];
        let amount: i32 = line[1..].parse().expect("Invalid number");

        // Update `pos` based on `dir` and `amount`
        // Remember: The dial has numbers 0-99.
        // Left (L) is toward lower numbers (subtraction).
        // Right (R) is toward higher numbers (addition).
        match dir {
            "L" => pos -= amount,
            "R" => pos += amount,
            _ => panic!("Invalid direction"),
        }

        // Update `pos` to be in the range 0-99
        pos = pos.rem_euclid(100);

        // Count the number of times the dial returns to 0
        if pos == 0 {
            count += 1;
        }
    }

    count
}

fn solve_part2(input: &str) -> u32 {
    let mut pos: i32 = 50;
    let mut count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse direction and amount
        let dir = &line[0..1];
        let amount: i32 = line[1..].parse().expect("Invalid number");

        // Update `pos` based on `dir` and `amount` and count zero crossings.
        // For Right (R), we count multiples of 100 in [pos, pos + amount].
        // For Left (L), we count multiples of 100 in [pos - amount, pos].
        //     using pos-1 to handle leaving and landing on 0
        match dir {
            "R" => {
                count += ((pos + amount).div_euclid(100) - pos.div_euclid(100)) as u32;
                pos = (pos + amount).rem_euclid(100);
            }
            "L" => {
                count += ((pos - 1).div_euclid(100) - (pos - 1 - amount).div_euclid(100)) as u32;
                pos = (pos - amount).rem_euclid(100);
            }
            _ => panic!("Invalid direction"),
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(solve_part1(input), 3);
    }

    #[test]
    fn test_example2() {
        /*Following the same rotations as in the above example, the dial points at zero a few extra times during its rotations:

        The dial starts by pointing at 50.
        The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
        The dial is rotated L30 to point at 52.
        The dial is rotated R48 to point at 0.
        The dial is rotated L5 to point at 95.
        The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
        The dial is rotated L55 to point at 0.
        The dial is rotated L1 to point at 99.
        The dial is rotated L99 to point at 0.
        The dial is rotated R14 to point at 14.
        The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.
        In this example, the dial points at 0 three times at the end of a rotation, plus three more times during a rotation. So, in this example, the new password would be 6.
         */
        let input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(solve_part2(input), 6);
    }

    #[test]
    fn test_example2_1() {
        /*Following the same rotations as in the above example, the dial points at zero a few extra times during its rotations:

        The dial starts by pointing at 50.
        The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
        The dial is rotated L30 to point at 52.
        The dial is rotated R48 to point at 0.
        The dial is rotated L5 to point at 95.
        The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
         */
        let input = "\
L68
L30
R48
L5
R60
";
        assert_eq!(solve_part2(input), 3);
    }

    #[test]
    fn test_div_euclid_sanity() {
        assert_eq!((-50_i32).div_euclid(100), -1);
        assert_eq!((-950_i32).div_euclid(100), -10);

        // Also check the remainders
        assert_eq!((-50_i32).rem_euclid(100), 50);
        assert_eq!((-950_i32).rem_euclid(100), 50);
    }

    #[test]
    fn test_turn_r1000() {
        // Be careful: if the dial were pointing at 50, a single rotation like R1000 would cause the dial
        //to point at 0 ten times before returning back to 50!

        let input = "\
    R1000
    ";
        assert_eq!(solve_part2(input), 10);
    }

    #[test]
    fn test_turn_l1000() {
        let input = "\
    L1000
    ";
        assert_eq!(solve_part2(input), 10);
    }

    #[test]
    fn test_turn_l50() {
        let input = "\
    L50
    ";
        assert_eq!(solve_part2(input), 1);
    }

    #[test]
    fn test_turn_r50() {
        let input = "\
    R50
    ";
        assert_eq!(solve_part2(input), 1);
    }

    #[test]
    fn test_turn_l50_l50() {
        let input = "\
    L50
    L50
    ";
        assert_eq!(solve_part2(input), 1);
    }

    #[test]
    fn test_turn_l50_r50() {
        let input = "\
    L50
    R50
    ";
        assert_eq!(solve_part2(input), 1);
    }

    #[test]
    fn test_turn_l50_l100() {
        let input = "\
    L50
    L100
    ";
        assert_eq!(solve_part2(input), 2);
    }

    #[test]
    fn test_turn_l150() {
        let input = "\
    L150
    ";
        assert_eq!(solve_part2(input), 2);
    }

    #[test]
    fn test_incorrect_guess() {
        // That's not the right answer; your answer is too high.
        // If you're stuck, make sure you're using the full input data; there are also some general tips on the about page, or you can ask for hints on the subreddit.
        let input = include_str!("../input.txt");
        assert_ne!(solve_part2(input), 6142);
    }
}
