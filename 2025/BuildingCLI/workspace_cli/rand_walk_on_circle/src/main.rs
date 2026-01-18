use rand::Rng;
use std::collections::{HashMap, HashSet};

/// Runs the simulation and returns the last number to be colored.
fn simulate_bug_on_clock() -> i32 {
    let mut visited = HashSet::new();
    let mut current_position = 12;

    visited.insert(current_position);
    let mut last_colored = current_position;
    let mut rng = rand::rng();

    while visited.len() < 12 {
        // Move clockwise or counter-clockwise
        if rng.random_bool(0.5) {
            current_position += 1;
        } else {
            current_position -= 1;
        }

        // Handle clock wrap-around
        if current_position > 12 {
            current_position = 1;
        } else if current_position < 1 {
            current_position = 12;
        }

        if !visited.contains(&current_position) {
            visited.insert(current_position);
            last_colored = current_position;
        }
    }
    last_colored
}

fn main() {
    let mut counts = HashMap::new();
    let runs = 10000;

    for _ in 0..runs {
        let result = simulate_bug_on_clock();
        *counts.entry(result).or_insert(0) += 1;
    }

    println!("Results for 100 runs:");
    for i in 1..=11 {
        println!("Number {}: {} times", i, counts.get(&i).unwrap_or(&0));
    }
}

// --- Test Module ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_number_is_never_start() {
        // Run it 1000 times to be statistically certain
        for _ in 0..1000 {
            let last = simulate_bug_on_clock();
            // The starting number (12) can never be the LAST one colored.
            assert!(
                last != 12,
                "The last number colored should not be the starting position 12!"
            );
        }
    }

    #[test]
    fn test_last_number_range() {
        let last = simulate_bug_on_clock();
        // Ensure the number is a valid clock number between 1 and 11
        assert!(last >= 1 && last <= 11);
    }
}
