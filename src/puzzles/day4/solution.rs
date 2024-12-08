use std::fs;

// Represents a direction to search in the grid
#[derive(Debug)]
struct Direction {
    dx: i32,
    dy: i32,
}

impl Direction {
    // All possible directions to search: horizontal, vertical, and diagonal
    fn all() -> Vec<Direction> {
        vec![
            Direction { dx: 1, dy: 0 },   // right
            Direction { dx: -1, dy: 0 },  // left
            Direction { dx: 0, dy: 1 },   // down
            Direction { dx: 0, dy: -1 },  // up
            Direction { dx: 1, dy: 1 },   // diagonal down-right
            Direction { dx: -1, dy: 1 },  // diagonal down-left
            Direction { dx: 1, dy: -1 },  // diagonal up-right
            Direction { dx: -1, dy: -1 }, // diagonal up-left
        ]
    }
}

fn main() {
    // Read the input file
    let contents = fs::read_to_string("puzzles/day4/input.txt")
        .expect("Should have been able to read the file");

    // Convert input into a 2D grid
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let count = count_xmas_occurrences(&grid);
    println!("Found {} occurrences of XMAS", count);
}

fn count_xmas_occurrences(grid: &[Vec<char>]) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let target = "XMAS";
    let mut count = 0;

    // Check every position as a potential starting point
    for y in 0..rows {
        for x in 0..cols {
            // Only proceed if we find an 'X'
            if grid[y][x] != 'X' {
                continue;
            }

            // Try all possible directions from this starting point
            for dir in Direction::all() {
                if check_word_in_direction(grid, x, y, &dir, target) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn check_word_in_direction(grid: &[Vec<char>], start_x: usize, start_y: usize, dir: &Direction, target: &str) -> bool {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let word_len = target.len();
    let target_chars: Vec<char> = target.chars().collect();

    // Check if the word would fit in this direction
    let end_x = start_x as i32 + dir.dx * (word_len as i32 - 1);
    let end_y = start_y as i32 + dir.dy * (word_len as i32 - 1);

    if end_x < 0 || end_x >= cols || end_y < 0 || end_y >= rows {
        return false;
    }

    // Check each character in the direction
    for i in 0..word_len {
        let current_x = (start_x as i32 + dir.dx * i as i32) as usize;
        let current_y = (start_y as i32 + dir.dy * i as i32) as usize;

        if grid[current_y][current_x] != target_chars[i] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_grid() {
        let input = vec![
            "MMMSXXMASM".chars().collect(),
            "MSAMXMSMSA".chars().collect(),
            "AMXSXMAAMM".chars().collect(),
            "MSAMASMSMX".chars().collect(),
            "XMASAMXAMM".chars().collect(),
            "XXAMMXXAMA".chars().collect(),
            "SMSMSASXSS".chars().collect(),
            "SAXAMASAAA".chars().collect(),
            "MAMMMXMMMM".chars().collect(),
            "MXMXAXMASX".chars().collect(),
        ];

        assert_eq!(count_xmas_occurrences(&input), 18);
    }
}