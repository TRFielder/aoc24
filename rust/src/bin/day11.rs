use std::time::Instant;

const INPUT: &str = include_str!("../../../inputs/day11.txt");

pub fn main() {
    let part1_start = Instant::now();
    let part1_result = part1(INPUT);
    let part1_duration = part1_start.elapsed();

    println!(
        "The result for day 10 part 1 is {}, it took {:?}",
        part1_result, part1_duration
    );
}

fn part1(input: &str) -> i64 {
    let mut array: Vec<i64> = input
        .split_whitespace()
        .map(|val| return val.parse::<i64>().unwrap())
        .collect();

    for _ in 0..25 {
        array = blink(array);
    }
    array.len() as i64
}

fn blink(stones: Vec<i64>) -> Vec<i64> {
    // Create a new vector to hold our modified stones
    let mut new_stones: Vec<i64> = Vec::new();

    // Iterate over the stones and apply the rules, in order.
    for stone in stones.iter() {
        // Replace 0 with 1 in the new vector
        if stone == &0i64 {
            new_stones.push(1);
            // Skip further rules
            continue;
        }

        // If there are an even number of digits...
        let stone_str = stone.to_string();
        if stone_str.len() % 2 == 0 {
            // Split the number down the middle
            let mid = stone_str.len() / 2;
            let left: i64 = stone_str[..mid].parse().unwrap();
            let right: i64 = stone_str[mid..].parse().unwrap();

            // Add the split numbers to the new vector
            new_stones.push(left);
            new_stones.push(right);

            // Skip further rules
            continue;
        }

        // If the other rules don't apply, multiply stone by 2024
        new_stones.push(stone * 2024);
    }

    new_stones
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../../testInputs/day11.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 55312);
    }
}
