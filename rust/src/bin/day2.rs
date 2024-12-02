const INPUT: &str = include_str!("../../../inputs/day2.txt");

fn main() {
    println!("The result for day 2 part 1 is {}", part1(INPUT));
    println!("The result for day 2 part 2 is {}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    let reports = input.lines();

    let mut safe: i32 = 0;

    // safe if all increasing or all decreasing

    for level in reports {
        let levels: Vec<i32> = level
            .split_whitespace()
            .map(|item| item.parse::<i32>().unwrap())
            .collect();

        // Check if all decreasing or all increasing.
        // Use zip to 'create' an iterator of all the elements after the first one
        // Also check if abs difference between adjacent values is between 1 and 3
        let is_decreasing_and_safe = levels
            .iter()
            .zip(levels.iter().skip(1))
            .all(|(a, b)| a > b && (a - b).abs() >= 1 && (a - b).abs() <= 3);

        let is_increasing_and_safe = levels
            .iter()
            .zip(levels.iter().skip(1))
            .all(|(a, b)| a < b && (a - b).abs() >= 1 && (a - b).abs() <= 3);

        if is_decreasing_and_safe || is_increasing_and_safe {
            safe += 1;
        }
    }
    safe
}

// Filthy - copy and paste the whole thing from part 1 into a disgusting one-liner. Let's not talk about this
fn is_safe(levels: &[i32]) -> bool {
    levels
        .iter()
        .zip(levels.iter().skip(1))
        .all(|(a, b)| a > b && (a - b).abs() >= 1 && (a - b).abs() <= 3)
        || levels
            .iter()
            .zip(levels.iter().skip(1))
            .all(|(a, b)| a < b && (a - b).abs() >= 1 && (a - b).abs() <= 3)
}

fn part2(input: &str) -> i32 {
    let reports = input.lines();

    let mut safe: i32 = 0;

    for level in reports {
        let levels: Vec<i32> = level
            .split_whitespace()
            .map(|item| item.parse::<i32>().unwrap())
            .collect();

        // Check if it's already safe, mark and and move on if so (ie don't check the next bit)
        if is_safe(&levels) {
            safe += 1;
            continue;
        }

        // Check if removing one element makes it safe
        for i in 0..levels.len() {
            let mut temp = levels.clone();
            temp.remove(i);
            // is it safe with the ith element removed?
            if is_safe(&temp) {
                safe += 1;
                // No need to keep looking, we only need to prove it once
                break;
            }
        }
    }

    safe
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("../../../testInputs/day2.txt");
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4);
    }
}
