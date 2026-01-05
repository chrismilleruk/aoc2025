use std::fs;

fn main() {
    let input =
        fs::read_to_string("day-01/input.txt").expect("Should have been able to read the file");
    let part1 = solve_part1(&input);
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

        pos = pos.rem_euclid(100);

        // Count the number of times the dial returns to 0
        if pos == 0 {
            count += 1;
        }
    }

    count
}

fn solve_part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
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
}
