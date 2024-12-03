const INPUT: &str = include_str!("../../../inputs/day3.txt");

use regex::Regex;

fn main() {
    println!("The result for day 3 part 1 is {}", part1(INPUT));
    println!("The result for day 3 part 2 is {}", part2(INPUT));
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

fn part2(input: &str) -> i32 {
    let regex: Regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let do_or_dont: Regex = Regex::new(r"(don't\(\)|do\(\))").unwrap();

    // We start off assuming we are in do() mode
    let mut do_multiply = true;
    let mut matches: Vec<(i32, i32)> = Vec::new();

    let mut mode_switch_index = 0;

    for regex_match in regex.captures_iter(input) {
        let match_start_index = regex_match.get(0).unwrap().start();

        // Check if there are any mode changing instructions between the last mode switch and this match
        let text_between = &input[mode_switch_index..match_start_index];

        // Update mode if we need to, do it in a loop and only the last one actually matters
        // yes I know this is inefficient. Don't judge me my brain can't handle the efficient way
        for mode_match in do_or_dont.captures_iter(text_between) {
            match mode_match.get(0).unwrap().as_str() {
                "don't()" => do_multiply = false,
                "do()" => do_multiply = true,
                _ => {}
            }
        }

        // Update the mode switch index to show where we last looked for an instruction
        mode_switch_index = match_start_index;

        // Only push multiplication matches if the last instruction was "do()"
        if do_multiply {
            let num1 = regex_match.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let num2 = regex_match.get(2).unwrap().as_str().parse::<i32>().unwrap();
            matches.push((num1, num2))
        }
    }

    let mut result: i32 = 0;

    for (a, b) in &matches {
        result += a * b;
    }

    result
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("../../../testInputs/day3.txt");
    const TEST_PART2_INPUT: &str = include_str!("../../../testInputs/day3_2.txt");
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_PART2_INPUT), 48);
    }
}
