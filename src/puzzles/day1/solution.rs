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
    left.sort_unstable();
    right.sort_unstable();
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

pub fn calculate_similarity_score(left: &[i32], right: &[i32]) -> i64 {
    left.iter()
        .map(|&num| {
            let count = right.iter().filter(|&&x| x == num).count() as i64;
            (num as i64) * count
        })
        .sum()
}

fn solve_puzzle_part1(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);
    calculate_total_distance(&mut left, &mut right)
}

fn solve_puzzle_part2(input: &str) -> i64 {
    let (left, right) = parse_input(input);
    calculate_similarity_score(&left, &right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        assert_eq!(solve_puzzle_part1(input), 11);
    }

    #[test]
    fn test_example_part2() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        assert_eq!(solve_puzzle_part2(input), 31);
    }
}

fn main() {
    // Print current directory for debugging
    println!("Current directory: {:?}", std::env::current_dir().unwrap());

    // Read the input file
    let input = std::fs::read_to_string("puzzles/day1/input.txt")
        .expect("Failed to read input file");

    let result_part1 = solve_puzzle_part1(&input);
    println!("Part 1 - Total distance: {}", result_part1);

    let result_part2 = solve_puzzle_part2(&input);
    println!("Part 2 - Similarity score: {}", result_part2);
}