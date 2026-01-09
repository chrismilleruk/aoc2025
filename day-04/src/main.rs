fn main() {
    let input = include_str!("../input.txt");

    let part1 = solve_part1(input);
    println!("Part 1 - Accessible rolls (Grid): {}", part1);

    let part1_bits = solve_part1_bitpacked(input);
    println!("Part 1 - Accessible rolls (Bitpacked): {}", part1_bits);

    let part2 = solve_part2(input);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // Problem:
    // The forklifts can only access a roll of paper if there are fewer than four
    // rolls of paper in the eight adjacent positions. If you can figure out which
    // rolls of paper the forklifts can access, they'll spend less time looking and
    // more time breaking down the wall to the cafeteria.
    // How many rolls of paper can be accessed by a forklift?

    // Restated:
    // - For each cell with a roll,
    // - count the surrounding 8 cells,
    // - if there are 4 or more than 4,
    // - then the roll is inaccessible.
    // - Return the number of accessible rolls.

    // Strategy
    // convert each line to
    // - a 2d grid
    // - AND a lookup for the number of rolls within a 3x window
    // - Iterate over the grid,
    // - For each cell with a roll,
    //   - Sum the lookup for this line and above and below.
    //   - If the sum is 4 or less (accounting for the cell itself),
    //     - then the roll is accessible.
    // - Return the number of accessible rolls.

    // Create a lookup for the number of rolls within a 3-column window
    let three_col_sums: Vec<Vec<u8>> = grid
        .iter()
        .map(|row| {
            let cols = row.len();
            let mut row_sums = vec![0u8; cols];
            for (c, _cell) in row.iter().enumerate() {
                let start = c.saturating_sub(1);
                let end = (c + 1).min(cols.saturating_sub(1));
                row_sums[c] = row[start..=end].iter().filter(|&&ch| ch == '@').count() as u8;
            }
            row_sums
        })
        .collect();

    let mut accessible_count = 0;
    let rows = grid.len();

    let _adjacent_counts: Vec<Vec<u8>> = (0..rows)
        .map(|r| {
            let cols = grid[r].len();
            (0..cols)
                .map(|c| {
                    if grid[r][c] == '.' {
                        return 0;
                    }

                    let mut sum_3x3 = three_col_sums[r][c] as u32;
                    if r > 0 {
                        sum_3x3 += three_col_sums[r - 1][c] as u32;
                    }
                    if r + 1 < rows {
                        sum_3x3 += three_col_sums[r + 1][c] as u32;
                    }

                    let adjacent = sum_3x3 - 1;
                    if adjacent < 4 {
                        accessible_count += 1;
                    }
                    adjacent as u8
                })
                .collect()
        })
        .collect();

    // println!("adjacent_counts: {:#?}", adjacent_counts);

    accessible_count
}

fn solve_part1_bitpacked(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }
    let rows = lines.len();
    let cols = lines[0].len();

    // Stride: 3 u64s per row (192 bits) is enough for 139 bits + padding
    let words_per_row = 3;
    let mut bitset = vec![0u64; rows * words_per_row];

    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.bytes().enumerate() {
            if ch == b'@' {
                // Offset by 1 bit to provide padding on the left
                let total_c = c + 1;
                bitset[r * words_per_row + (total_c / 64)] |= 1 << (total_c % 64);
            }
        }
    }

    let mut accessible_count = 0;
    for r in 0..rows {
        for c in 0..cols {
            // Check current cell (at offset c + 1)
            let total_c = c + 1;
            let word_idx = r * words_per_row + (total_c / 64);
            let bit_idx = total_c % 64;
            if (bitset[word_idx] >> bit_idx) & 1 == 0 {
                continue;
            }

            let mut neighbors = 0;
            for dr in -1..=1 {
                let nr = r as i32 + dr;
                if nr < 0 || nr >= rows as i32 {
                    continue;
                }
                let nr = nr as usize;

                // Load 128 bits into a window centered around bit index 'total_c'
                // We want bits [total_c - 1, total_c, total_c + 1]
                // Shift by total_c - 1 to align bit total_c - 1 to position 0
                let start_bit = total_c - 1;
                let col_idx = start_bit / 64;
                let bit_offset = start_bit % 64;

                let word0 = bitset[nr * words_per_row + col_idx];
                let word1 = if col_idx + 1 < words_per_row {
                    bitset[nr * words_per_row + col_idx + 1]
                } else {
                    0
                };
                let combined = (word0 as u128) | ((word1 as u128) << 64);
                let win = (combined >> bit_offset) as u64;

                // With padding at c=0 (bit 1) and zeros at the end of the row buffer,
                // we can safely use a constant mask of 0b111.
                neighbors += (win & 0b111).count_ones();
            }

            if neighbors.saturating_sub(1) < 4 {
                accessible_count += 1;
            }
        }
    }
    accessible_count
}

fn solve_part2(_input: &str) -> usize {
    // Placeholder logic - NOT SOLVING IT
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../example1.txt");

        // Per the problem description, 13 rolls are accessible.
        assert_eq!(solve_part1(input), 13);
    }

    #[test]
    fn test_part2() {
        // Placeholder for part 2 test
        // let input = include_str!("../example1.txt");
        // assert_eq!(solve_part2(input), 0);
    }
}
