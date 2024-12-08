use std::fs;

fn main() {
    let contents = fs::read_to_string("puzzles/day4/input.txt")
        .expect("Should have been able to read the file");

    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let count = count_x_mas_occurrences(&grid);
    println!("Found {} X-MAS patterns", count);
}

// Check if a sequence forms MAS (forwards or backwards)
fn is_mas_sequence(chars: &[char]) -> bool {
    chars == ['M', 'A', 'S'] || chars == ['S', 'A', 'M']
}

fn count_x_mas_occurrences(grid: &[Vec<char>]) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // We need at least one cell padding on all sides for a valid X pattern
    for y in 1..rows-1 {
        for x in 1..cols-1 {
            // Center must be 'A'
            if grid[y][x] != 'A' {
                continue;
            }

            // Check X pattern: top-left to bottom-right crossing top-right to bottom-left
            let pattern1 = [
                grid[y-1][x-1],  // top-left
                grid[y][x],      // center
                grid[y+1][x+1]   // bottom-right
            ];

            let pattern2 = [
                grid[y-1][x+1],  // top-right
                grid[y][x],      // center
                grid[y+1][x-1]   // bottom-left
            ];

            if is_mas_sequence(&pattern1) && is_mas_sequence(&pattern2) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_grid() {
        let input = vec![
            ".M.S......".chars().collect(),
            "..A..MSMS.".chars().collect(),
            ".M.S.MAA..".chars().collect(),
            "..A.ASMSM.".chars().collect(),
            ".M.S.M....".chars().collect(),
            "..........".chars().collect(),
            "S.S.S.S.S.".chars().collect(),
            ".A.A.A.A..".chars().collect(),
            "M.M.M.M.M.".chars().collect(),
            "..........".chars().collect(),
        ];

        assert_eq!(count_x_mas_occurrences(&input), 9);
    }

    #[test]
    fn test_simple_x_mas() {
        let input = vec![
            "M.S".chars().collect(),
            ".A.".chars().collect(),
            "M.S".chars().collect(),
        ];

        assert_eq!(count_x_mas_occurrences(&input), 1);
    }
}