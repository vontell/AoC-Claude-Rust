use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let input = read_input("puzzles/day2/input.txt");
    let reports = parse_reports(&input);
    let safe_count = count_safe_reports(&reports);
    println!("Number of safe reports: {}", safe_count);
}

fn read_input(filepath: &str) -> String {
    read_to_string(Path::new(filepath))
        .unwrap_or_else(|e| panic!("Error reading file: {}", e))
}

fn parse_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe_report(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut increasing: Option<bool> = None;

    for window in levels.windows(2) {
        let diff = window[1] - window[0];

        // Check if difference is between 1 and 3 (inclusive)
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        match increasing {
            None => {
                // First pair determines if sequence is increasing or decreasing
                increasing = Some(diff > 0);
            }
            Some(is_increasing) => {
                // Check if current difference matches the pattern
                if (diff > 0) != is_increasing {
                    return false;
                }
            }
        }
    }

    true
}

fn count_safe_reports(reports: &[Vec<i32>]) -> usize {
    reports.iter().filter(|report| is_safe_report(report)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe_report() {
        assert!(is_safe_report(&[7, 6, 4, 2, 1]), "Should be safe (decreasing)");
        assert!(!is_safe_report(&[1, 2, 7, 8, 9]), "Should be unsafe (gap too large)");
        assert!(!is_safe_report(&[9, 7, 6, 2, 1]), "Should be unsafe (gap too large)");
        assert!(!is_safe_report(&[1, 3, 2, 4, 5]), "Should be unsafe (changes direction)");
        assert!(!is_safe_report(&[8, 6, 4, 4, 1]), "Should be unsafe (no change)");
        assert!(is_safe_report(&[1, 3, 6, 7, 9]), "Should be safe (increasing)");
    }
}