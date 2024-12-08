use std::fs;
use regex::Regex;

fn main() {
    // Read the input file
    let content = fs::read_to_string("puzzles/day3/input.txt")
        .expect("Failed to read input file");

    // Calculate and print the result
    let result = process_memory(&content);
    println!("Sum of all enabled multiplication results: {}", result);
}

fn process_memory(content: &str) -> i64 {
    // Create patterns for instructions
    let mul_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_pattern = Regex::new(r"do\(\)").unwrap();
    let dont_pattern = Regex::new(r"don't\(\)").unwrap();

    // Find all instruction positions and their types
    let mut instructions = Vec::new();

    // Find multiplication instructions
    for cap in mul_pattern.captures_iter(content) {
        let start = cap.get(0).unwrap().start();
        let num1: i64 = cap[1].parse().unwrap();
        let num2: i64 = cap[2].parse().unwrap();
        instructions.push((start, Instruction::Mul(num1, num2)));
    }

    // Find do() instructions
    for m in do_pattern.find_iter(content) {
        instructions.push((m.start(), Instruction::Do));
    }

    // Find don't() instructions
    for m in dont_pattern.find_iter(content) {
        instructions.push((m.start(), Instruction::Dont));
    }

    // Sort instructions by position to process them in order
    instructions.sort_by_key(|&(pos, _)| pos);

    // Process instructions in order
    let mut enabled = true;  // Multiplications start enabled
    let mut sum = 0;

    for (_, instruction) in instructions {
        match instruction {
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            Instruction::Mul(num1, num2) => {
                if enabled {
                    sum += num1 * num2;
                }
            }
        }
    }

    sum
}

// Enum to represent different types of instructions
#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mul(i64, i64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conditional_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)do()?mul(8,5))";
        assert_eq!(process_memory(input), 48);  // 2*4 + 8*5
    }

    #[test]
    fn test_basic_enabling() {
        let input = "mul(2,3)don't()mul(4,5)do()mul(6,7)";
        assert_eq!(process_memory(input), 48);  // 2*3 + 6*7
    }

    #[test]
    fn test_all_disabled() {
        let input = "don't()mul(2,3)mul(4,5)mul(6,7)";
        assert_eq!(process_memory(input), 0);
    }

    #[test]
    fn test_all_enabled() {
        let input = "do()mul(2,3)mul(4,5)";
        assert_eq!(process_memory(input), 26);  // 2*3 + 4*5
    }
}