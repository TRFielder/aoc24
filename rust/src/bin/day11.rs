use std::{collections::HashMap, time::Instant};

const INPUT: &str = include_str!("../../../inputs/day11.txt");

pub fn main() {
    let part1_start = Instant::now();
    let part1_result = part1(INPUT);
    let part1_duration = part1_start.elapsed();

    println!(
        "The result for day 11 part 1 is {}, it took {:?}",
        part1_result, part1_duration
    );

    let part2_start = Instant::now();
    let part2_result = part2(INPUT);
    let part2_duration = part2_start.elapsed();

    println!(
        "The result for day 11 part 2 is {}, it took {:?}",
        part2_result, part2_duration
    );
}

fn part1(input: &str) -> i64 {
    // We can use a hashmap because every stone has the same outcome from the rules!
    let mut stones: HashMap<i64, i64> = HashMap::new();

    for stone in input.split_whitespace() {
        let stone_num: i64 = stone.parse().unwrap();
        // So we count how many of each stone there are
        *stones.entry(stone_num).or_default() += 1;
    }

    for _ in 0..25 {
        stones = blink(stones);
    }
    stones.values().sum()
}

fn part2(input: &str) -> i64 {
    // We can use a hashmap because every stone has the same outcome from the rules!
    let mut stones: HashMap<i64, i64> = HashMap::new();

    for stone in input.split_whitespace() {
        let stone_num: i64 = stone.parse().unwrap();
        // So we count how many of each stone there are
        *stones.entry(stone_num).or_default() += 1;
    }

    for _ in 0..75 {
        stones = blink(stones);
    }
    stones.values().sum()
}

fn blink(stones: HashMap<i64, i64>) -> HashMap<i64, i64> {
    // Create a new hashmap to hold our modified stones
    let mut new_stones: HashMap<i64, i64> = HashMap::new();

    // Iterate over the stones and apply the rules, in order.
    for (stone, count) in stones {
        // Replace 0 with 1 in the new vector
        if stone == 0 {
            *new_stones.entry(1).or_default() += count;
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

            // Add the split numbers to the new hashmap
            *new_stones.entry(left).or_default() += count;
            *new_stones.entry(right).or_default() += count;

            // Skip further rules
            continue;
        }

        // If the other rules don't apply, multiply stone by 2024
        *new_stones.entry(stone * 2024).or_default() += count;
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

    // No need to test for part 2. It's the same thing just longer...
}
