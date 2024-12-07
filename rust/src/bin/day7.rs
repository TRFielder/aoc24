use std::{collections::HashMap, time::Instant};

const INPUT: &str = include_str!("../../../inputs/day7.txt");

pub fn main() {
    // We're gonna time every solution from now on cos it's fun
    let part1_start = Instant::now();
    let part1_result = part1(INPUT);
    let part1_duration = part1_start.elapsed();

    println!(
        "The result for day 6 part 1 is {}, it took {:?}",
        part1_result, part1_duration
    );
}

fn read_input_to_map(input: &str) -> HashMap<i64, Vec<i64>> {
    let mut map: HashMap<i64, Vec<i64>> = HashMap::new();

    for line in input.lines() {
        let (target, values) = line.split_once(":").unwrap();

        // Split the values by their whitespace now
        let input_values: Vec<i64> = values
            .trim()
            .split_whitespace()
            .map(|number| number.parse::<i64>().unwrap())
            .collect();

        println!("the number is {}", target);

        map.insert(target.parse::<i64>().unwrap(), input_values);
    }
    map
}

// Recursive function to work through the nums vector and try to reach target
fn can_reach_target_value(
    target: &i64,
    nums: &Vec<i64>,
    current_value: &i64,
    index: usize,
) -> bool {
    // If we've used all nums, check if current value equals target
    if index == nums.len() as usize {
        return current_value == target;
    }

    // add the next number in the list
    let add = can_reach_target_value(target, nums, &(current_value + nums[index]), index + 1);

    // multiply the next number in the list
    let multiply = can_reach_target_value(target, nums, &(current_value * nums[index]), index + 1);

    // If add or multiply can reach target value at some point, this will return true all the way up to the initial function call
    add || multiply
}

// Largest input is bigger than i32 can hold
fn part1(input: &str) -> i64 {
    let map = read_input_to_map(input);

    let mut result: i64 = 0;
    // For an item in the map
    for (target, nums) in map.iter() {
        if can_reach_target_value(target, nums, &0, 0) {
            result += target
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../../testInputs/day7.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3749);
    }
}
