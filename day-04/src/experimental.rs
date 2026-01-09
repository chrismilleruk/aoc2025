use rayon::prelude::*;

/// The "Autovectorized" version (Scalar)
pub fn solve_part1_autovectorized(input: &str) -> usize {
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    if lines.is_empty() {
        return 0;
    }
    let (rows, cols) = (lines.len(), lines[0].len());
    let mut grid = vec![0u8; (rows + 2) * (cols + 2)];
    for r in 0..rows {
        for c in 0..cols {
            if lines[r][c] == b'@' {
                grid[(r + 1) * (cols + 2) + (c + 1)] = 1;
            }
        }
    }
    let mut count = 0;
    let stride = cols + 2;
    for r in 1..=rows {
        let row_idx = r * stride;
        let prev_row = row_idx - stride;
        let next_row = row_idx + stride;
        for c in 1..=cols {
            if grid[row_idx + c] == 1 {
                let s = grid[prev_row + c - 1]
                    + grid[prev_row + c]
                    + grid[prev_row + c + 1]
                    + grid[row_idx + c - 1]
                    + grid[row_idx + c + 1]
                    + grid[next_row + c - 1]
                    + grid[next_row + c]
                    + grid[next_row + c + 1];
                if s < 4 {
                    count += 1;
                }
            }
        }
    }
    count
}

/// The "Bitpacked" version (Scalar)
pub fn solve_part1_bitpacked(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }
    let (rows, cols) = (lines.len(), lines[0].len());
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
    let mut count = 0;
    for r in 0..rows {
        for c in 0..cols {
            let tc = c + 1;
            if (bitset[r * words_per_row + (tc / 64)] >> (tc % 64)) & 1 == 0 {
                continue;
            }
            let mut neighbors = 0;
            for dr in -1..=1 {
                let nr = r as i32 + dr;
                if nr < 0 || nr >= rows as i32 {
                    continue;
                }
                let nr = nr as usize;
                let sb = tc - 1;
                let ci = sb / 64;
                let bo = sb % 64;
                let w0 = bitset[nr * words_per_row + ci];
                let w1 = if ci + 1 < words_per_row {
                    bitset[nr * words_per_row + ci + 1]
                } else {
                    0
                };
                let combined = (w0 as u128) | ((w1 as u128) << 64);
                neighbors += ((combined >> bo) as u64 & 0b111).count_ones();
            }
            if neighbors.saturating_sub(1) < 4 {
                count += 1;
            }
        }
    }
    count
}

/// The "Bit-Parallel SWAR" version (Multi-threaded)
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

                macro_rules! csa {
                    ($a:expr, $b:expr, $c:expr) => {
                        ($a ^ $b ^ $c, ($a & $b) | ($b & $c) | ($a & $c))
                    };
                }
                let (s1_0, c1_0) = csa!(ns[0], ns[1], ns[2]);
                let (s1_1, c1_1) = csa!(ns[3], ns[4], ns[5]);
                let (s1_2, c1_2) = csa!(ns[6], ns[7], 0);

                let (_s2_0, c2_0) = csa!(s1_0, s1_1, s1_2);
                let (s2_1, c2_1) = csa!(c1_0, c1_1, c1_2);
                let bit_2 = c2_1 | (s2_1 & c2_0);

                let accessible_mask = b & !bit_2;
                row_count += accessible_mask.count_ones() as usize;
            }
            row_count
        })
        .sum()
}

/// The "Bit-Parallel SWAR" version for Part 2 (Single-threaded)
/// Iteratively removes accessible rolls until stabilization.
pub fn solve_part2_swar(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }
    let rows = lines.len();
    let words_per_row = 3;
    let mut bitset = vec![0u64; rows * words_per_row];

    let mut initial_count = 0;
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.bytes().enumerate() {
            if ch == b'@' {
                let tc = c + 1;
                bitset[r * words_per_row + (tc / 64)] |= 1 << (tc % 64);
                initial_count += 1;
            }
        }
    }

    loop {
        let mut changed = false;
        let mut new_bitset = bitset.clone();

        for r in 0..rows {
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
                    let b_idx = nr as usize * words_per_row;
                    let w0 = bitset[b_idx + w];
                    let wp = if w > 0 { bitset[b_idx + w - 1] } else { 0 };
                    let wn = if w + 1 < words_per_row {
                        bitset[b_idx + w + 1]
                    } else {
                        0
                    };
                    let low = (w0 << 1) | (wp >> 63);
                    let high = (w0 >> 1) | (wn << 63);
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

                macro_rules! fa {
                    ($a:expr, $b:expr, $c:expr) => {
                        (($a ^ $b ^ $c), ($a & $b) | ($b & $c) | ($a & $c))
                    };
                }
                let (s0a, s1a) = fa!(ns[0], ns[1], ns[2]);
                let (s0b, s1b) = fa!(ns[3], ns[4], ns[5]);
                let (s0c, s1c) = fa!(ns[6], ns[7], 0);
                let (_s0_f, s1_f0) = fa!(s0a, s0b, s0c);
                let (s1_f1, s2_f1) = fa!(s1a, s1b, s1c);
                let (_, c1_t2) = fa!(s1_f0, s1_f1, 0);
                let (s2_f, s3_f) = fa!(s2_f1, c1_t2, 0);

                let accessible_mask = b & !(s2_f | s3_f);
                if accessible_mask != 0 {
                    new_bitset[r * words_per_row + w] &= !accessible_mask;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
        bitset = new_bitset;
    }

    let final_count: usize = bitset.iter().map(|w| w.count_ones() as usize).sum();
    initial_count - final_count
}

/// The "Bit-Parallel SWAR" version for Part 1 (Single-threaded)
/// Processes 64 cells at a time using bitwise logic gates.
pub fn solve_part1_swar_scalar(input: &str) -> usize {
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

    let mut count = 0;
    for r in 0..rows {
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
                let w1 = if w > 0 { bitset[base_idx + w - 1] } else { 0 };
                let w2 = if w + 1 < words_per_row {
                    bitset[base_idx + w + 1]
                } else {
                    0
                };
                let low = (w0 << 1) | (w1 >> 63);
                let high = (w0 >> 1) | (w2 << 63);
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

            macro_rules! csa {
                ($a:expr, $b:expr, $c:expr) => {
                    ($a ^ $b ^ $c, ($a & $b) | ($b & $c) | ($a & $c))
                };
            }
            let (s1_0, c1_0) = csa!(ns[0], ns[1], ns[2]);
            let (s1_1, c1_1) = csa!(ns[3], ns[4], ns[5]);
            let (s1_2, c1_2) = csa!(ns[6], ns[7], 0);

            let (_s2_0, c2_0) = csa!(s1_0, s1_1, s1_2);
            let (s2_1, c2_1) = csa!(c1_0, c1_1, c1_2);
            let bit_2 = c2_1 | (s2_1 & c2_0);

            count += (b & !bit_2).count_ones() as usize;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

    #[test]
    fn test_auto() {
        assert_eq!(solve_part1_autovectorized(EX), 13);
    }
    #[test]
    fn test_bits() {
        assert_eq!(solve_part1_bitpacked(EX), 13);
    }
    #[test]
    fn test_swar() {
        assert_eq!(solve_part1_parallel_swar(EX), 13);
    }
    #[test]
    fn test_p2() {
        assert_eq!(solve_part2_swar(EX), 43);
    }
}
