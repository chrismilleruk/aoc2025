use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let result = solve(&input);
    println!("Password: {}", result);
}

fn solve(input: &str) -> u32 {
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

        // TODO: Implement rotation logic here
        // Update `pos` based on `dir` and `amount`
        // Remember: The dial has numbers 0-99.
        // Left (L) is toward lower numbers (subtraction).
        // Right (R) is toward higher numbers (addition).
        
        // Hint: In Rust, `%` is the remainder operator, not modulus. 
        // For negative numbers, you often want `rem_euclid`.
        // e.g., (-5 % 100) is -5, but (-5_i32).rem_euclid(100) is 95.

        if pos == 0 {
            count += 1;
        }
    }

    count
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
        assert_eq!(solve(input), 3);
    }
}
