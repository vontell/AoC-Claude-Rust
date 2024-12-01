pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() == 2 {
            if let (Ok(l), Ok(r)) = (numbers[0].parse(), numbers[1].parse()) {
                left.push(l);
                right.push(r);
            }
        }
    }

    (left, right)
}

pub fn calculate_total_distance(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    // Sort both lists
    left.sort_unstable();
    right.sort_unstable();

    // Zip the sorted lists and calculate differences
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

pub fn solve_puzzle(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);
    calculate_total_distance(&mut left, &mut right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "3   4
                     4   3
                     2   5
                     1   3
                     3   9
                     3   3";

        assert_eq!(solve_puzzle(input), 11);
    }
}

fn main() {
    // Print current directory for debugging
    println!("Current directory: {:?}", std::env::current_dir().unwrap());

    // Read the input file
    let input = std::fs::read_to_string("puzzles/day1/input.txt")
        .expect("Failed to read input file");

    let result = solve_puzzle(&input);
    println!("Total distance: {}", result);
    // Answer: 2756096
}