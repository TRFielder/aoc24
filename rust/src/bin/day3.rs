const INPUT: &str = include_str!("../../../inputs/day3.txt");

use regex::Regex;

fn main() {
    println!("The result for day 3 part 1 is {}", part1(INPUT));
}

fn part1(input: &str) -> i32 {
    let regex: Regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    // Try to parse through the input and find all the matches
    let out: Vec<(i32, i32)> = regex
        .captures_iter(input)
        .filter_map(|matches| {
            // parse the matches into integers
            let num1 = matches.get(1)?.as_str().parse::<i32>().unwrap();
            let num2 = matches.get(2)?.as_str().parse::<i32>().unwrap();
            Some((num1, num2))
        })
        .collect();

    let mut result: i32 = 0;

    for (a, b) in &out {
        result += a * b;
    }
    result
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("../../../testInputs/day3.txt");
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 161);
    }
}
