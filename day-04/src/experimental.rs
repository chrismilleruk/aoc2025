use rayon::prelude::*;

/// Experimental SIMD-friendly and high-performance implementations for Day 4.
///
/// This module explores auto-vectorization and manual SIMD techniques.

pub fn solve_part1_autovectorized(input: &str) -> usize {
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    if lines.is_empty() {
        return 0;
    }

    let rows = lines.len();
    let cols = lines[0].len();

    // Flatten grid to a single contiguous buffer for cache locality
    // We add padding (1 byte border) to avoid bounds checks in the hot loop
    let mut grid = vec![0u8; (rows + 2) * (cols + 2)];
    for r in 0..rows {
        for c in 0..cols {
            if lines[r][c] == b'@' {
                grid[(r + 1) * (cols + 2) + (c + 1)] = 1;
            }
        }
    }

    let mut accessible_count = 0;
    let stride = cols + 2;

    // The "Hot Loop": Structured for auto-vectorization
    // By using a flattened grid and a fixed stride, the compiler can
    // more easily use SIMD instructions to sum neighbors.
    for r in 1..=rows {
        let row_idx = r * stride;
        let prev_row = row_idx - stride;
        let next_row = row_idx + stride;

        for c in 1..=cols {
            if grid[row_idx + c] == 0 {
                continue;
            }

            // Sum 8 neighbors.
            // In a flattened 1D array with stride, these are fixed offsets.
            let neighbors = grid[prev_row + c - 1]
                + grid[prev_row + c]
                + grid[prev_row + c + 1]
                + grid[row_idx + c - 1]
                + grid[row_idx + c + 1]
                + grid[next_row + c - 1]
                + grid[next_row + c]
                + grid[next_row + c + 1];

            if neighbors < 4 {
                accessible_count += 1;
            }
        }
    }

    accessible_count
}

pub fn solve_part1_bitpacked(input: &str) -> usize {
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

/// The "Bit-Parallel SWAR" version (Multi-threaded)
/// Processes 64 cells at a time using bitwise logic gates and Rayon.
pub fn solve_part1_parallel_swar(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }
    let (rows, _cols) = (lines.len(), lines[0].len());
    let words_per_row = 3;
    let mut bitset = vec![0u64; rows * words_per_row];

    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.bytes().enumerate() {
            if ch == b'@' {
                let tc = c + 1;
                bitset[r * words_per_row + (tc / 64)] |= 1 << (tc % 64);
            }
        }
    }

    (0..rows)
        .into_par_iter()
        .map(|r| {
            let mut row_count = 0;
            for w in 0..words_per_row {
                let b = bitset[r * words_per_row + w];
                if b == 0 {
                    continue;
                }

                let mut ns = [0u64; 8];
                for dr in -1..=1 {
                    let nr = r as i32 + dr;
                    if nr < 0 || nr >= rows as i32 {
                        continue;
                    }
                    let base_idx = nr as usize * words_per_row;

                    let w0 = bitset[base_idx + w];
                    let w_prev = if w > 0 { bitset[base_idx + w - 1] } else { 0 };
                    let w_next = if w + 1 < words_per_row {
                        bitset[base_idx + w + 1]
                    } else {
                        0
                    };

                    let low = (w0 << 1) | (w_prev >> 63);
                    let high = (w0 >> 1) | (w_next << 63);

                    match dr {
                        -1 => {
                            ns[0] = low;
                            ns[1] = w0;
                            ns[2] = high;
                        }
                        0 => {
                            ns[3] = low;
                            ns[4] = high;
                        }
                        1 => {
                            ns[5] = low;
                            ns[6] = w0;
                            ns[7] = high;
                        }
                        _ => unreachable!(),
                    }
                }

                // Carry-Save Adder tree to sum 8 bitstreams
                macro_rules! csa {
                    ($a:expr, $b:expr, $c:expr) => {
                        ($a ^ $b ^ $c, ($a & $b) | ($b & $c) | ($a & $c))
                    };
                }

                let (s1_0, c1_0) = csa!(ns[0], ns[1], ns[2]);
                let (s1_1, c1_1) = csa!(ns[3], ns[4], ns[5]);
                let (s1_2, c1_2) = csa!(ns[6], ns[7], 0);

                // Level 2
                let (s2_0, c2_0) = csa!(s1_0, s1_1, s1_2);
                let (s2_1, c2_1) = csa!(c1_0, c1_1, c1_2);

                // s2_0 is the bit 0 of the sum.
                // s2_1 and c2_0 both represent bit 1 of the sum.
                // c2_1 represents bit 2 of the sum.

                // We want sum < 4. Sum is >= 4 if bit 2 is 1 OR both components of bit 1 is 1.
                // i.e., bit_2 | (s2_1 & c2_0)
                let bit_2 = c2_1 | (s2_1 & c2_0);

                let accessible_mask = b & !bit_2;
                row_count += accessible_mask.count_ones() as usize;
            }
            row_count
        })
        .sum()
}

/// A "Bit-Sliced" approach (conceptually)
/// This simulates how you would process 8 bit-streams in parallel.
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "..@@.@@@@.\n\
                           @@@.@.@.@@\n\
                           @@@@@.@.@@\n\
                           @.@@@@..@.\n\
                           @@.@@@@.@@\n\
                           .@@@@@@@.@\n\
                           .@.@.@.@@@\n\
                           @.@@@.@@@@\n\
                           .@@@@@@@@.\n\
                           @.@.@@@.@.";

    #[test]
    fn test_autovectorized() {
        assert_eq!(solve_part1_autovectorized(EXAMPLE), 13);
    }

    #[test]
    fn test_bitpacked() {
        assert_eq!(solve_part1_bitpacked(EXAMPLE), 13);
    }

    #[test]
    fn test_parallel_swar() {
        assert_eq!(solve_part1_parallel_swar(EXAMPLE), 13);
    }
}
