use std::{collections::HashMap, time::Instant};

const INPUT: &str = include_str!("../../../inputs/day7.txt");

pub fn main() {
    // We're gonna time every solution from now on cos it's fun
    let part1_start = Instant::now();
    let part1_result = part1(INPUT);
    let part1_duration = part1_start.elapsed();

    println!(
        "The result for day 7 part 1 is {}, it took {:?}",
        part1_result, part1_duration
    );

    let part2_start = Instant::now();
    let part2_result = part2(INPUT);
    let part2_duration = part2_start.elapsed();

    println!(
        "The result for day 7 part 2 is {}, it took {:?}",
        part2_result, part2_duration
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

        map.insert(target.parse::<i64>().unwrap(), input_values);
    }
    map
}

// Recursive function to work through the nums vector and try to reach target
fn can_reach_target_value(
    with_concat: bool,
    target: &i64,
    nums: &Vec<i64>,
    current_value: &i64,
    index: usize,
) -> bool {
    // If our current value is bigger than the target let's skip execution to save time
    if current_value > target {
        return false;
    }

    // If we've used all nums, check if current value equals target
    if index == nums.len() as usize {
        return current_value == target;
    }

    // add the next number in the list
    let add = can_reach_target_value(
        with_concat,
        target,
        nums,
        &(current_value + nums[index]),
        index + 1,
    );

    // multiply the next number in the list
    let multiply = can_reach_target_value(
        with_concat,
        target,
        nums,
        &(current_value * nums[index]),
        index + 1,
    );

    // Concatenate with the next number in the list
    // WE CAN CONCAT MORE THAN ONCE ASIUDGALSUIDFGHSUIKLDHFGILSUDHGFO
    let concatenated = {
        let num_digits = nums[index].ilog10() + 1; // Number of digits in the current number
        current_value * 10i64.pow(num_digits as u32) + nums[index]
    };

    let concat: bool = if with_concat {
        can_reach_target_value(with_concat, target, nums, &concatenated, index + 1)
    } else {
        false
    };

    // If add, multiply or concat can reach target value at some point, this will return true all the way up to the initial function call
    add || multiply || concat
}

// Largest input is bigger than i32 can hold
fn part1(input: &str) -> i64 {
    let map = read_input_to_map(input);

    let mut result: i64 = 0;
    // For an item in the map
    for (target, nums) in map.iter() {
        if can_reach_target_value(false, target, nums, &0, 0) {
            result += target
        }
    }

    result
}

fn part2(input: &str) -> i64 {
    let map = read_input_to_map(input);

    let mut result: i64 = 0;

    for (target, nums) in map.iter() {
        if can_reach_target_value(true, target, nums, &0, 0) {
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 11387);
    }

    #[test]
    fn test_only_concat() {
        assert_eq!(
            can_reach_target_value(true, &156, &vec![15 as i64, 6 as i64], &0, 0),
            true
        );
    }
}
