use std::fs;
use regex::Regex;

fn main() {
    // Read the input file
    let content = fs::read_to_string("puzzles/day3/input.txt")
        .expect("Failed to read input file");

    // Calculate and print the result
    let result = process_memory(&content);
    println!("Sum of all multiplication results: {}", result);
}

fn process_memory(content: &str) -> i64 {
    // Create regex pattern for valid mul instructions
    // Pattern explanation:
    // - Must start with exactly "mul("
    // - First number is 1-3 digits
    // - Followed by exactly one comma
    // - Second number is 1-3 digits
    // - Must end with exactly ")"
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Find all valid matches and sum their multiplication results
    pattern.captures_iter(content)
        .map(|cap| {
            // Extract and parse the two numbers
            let num1: i64 = cap[1].parse().unwrap();
            let num2: i64 = cap[2].parse().unwrap();

            // Multiply them together
            num1 * num2
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(11,8)mul(8,5))";
        assert_eq!(process_memory(input), 161);
    }

    #[test]
    fn test_invalid_patterns() {
        let input = "mul(4* mul(6,9! ?(12,34) mul ( 2 , 4 )";
        assert_eq!(process_memory(input), 0);
    }

    #[test]
    fn test_valid_numbers() {
        let input = "mul(123,4)mul(44,46)";
        assert_eq!(process_memory(input), 2516); // 123*4 + 44*46
    }
}