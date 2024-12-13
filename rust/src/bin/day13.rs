use std::time::Instant;

use regex::Regex;

const INPUT: &str = include_str!("../../../inputs/day13.txt");

const COST_A: i64 = 3;
const COST_B: i64 = 1;
const PART_2_ADDITION: i64 = 10000000000000;

pub fn main() {
    let part1_start = Instant::now();
    let part1_result = part1(INPUT);
    let part1_duration = part1_start.elapsed();

    println!(
        "The result for day 13 part 1 is {}, it took {:?}",
        part1_result, part1_duration
    );

    let part2_start = Instant::now();
    let part2_result = part2(INPUT);
    let part2_duration = part2_start.elapsed();

    println!(
        "The result for day 13 part 2 is {}, it took {:?}",
        part2_result, part2_duration
    );
}

fn part1(input: &str) -> i64 {
    let games = parse_input(input);

    // For each game, a point is reachable if some multiple of x[0] and x[1] adds up to x[2]
    // Same for y values
    let mut output: i64 = 0;
    // add to the output for each game that can reach the target
    for game in games {
        let tokens = calculate_spend_to_target(game, false);
        output += tokens
    }

    output
}

fn part2(input: &str) -> i64 {
    let games = parse_input(input);

    // For each game, a point is reachable if some multiple of x[0] and x[1] adds up to x[2]
    // Same for y values
    let mut output: i64 = 0;
    // add to the output for each game that can reach the target
    for game in games {
        let tokens = calculate_spend_to_target(game, true);
        output += tokens
    }

    output
}

fn calculate_spend_to_target(game: Vec<(i64, i64)>, extra_distance: bool) -> i64 {
    // Get the x and y values
    let x0 = game[0].0;
    let x1 = game[1].0;
    let x_target = if !extra_distance {
        game[2].0
    } else {
        game[2].0 + PART_2_ADDITION
    };

    let y0 = game[0].1;
    let y1 = game[1].1;
    let y_target = if !extra_distance {
        game[2].1
    } else {
        game[2].1 + PART_2_ADDITION
    };

    // This is a linear algebra thing. Did it by hand
    // If the denominator is 0, there is no solution (and we don't want to divide by 0 anyway)
    if x0 * y1 - x1 * y0 == 0 {
        return 0;
    } else {
        // We need both a and b to be integers (can't press a button half a time)
        // I can't think of a way to do this efficiently so we're just running the calculation twice...
        let a_is_integer: bool = ((x_target * y1) - (x1 * y_target)) % ((x0 * y1) - (x1 * y0)) == 0;

        if a_is_integer {
            let a: i64 = ((x_target * y1) - (x1 * y_target)) / ((x0 * y1) - (x1 * y0));

            let b_is_integer: bool = (x_target - (a * x0)) % x1 == 0;

            if b_is_integer {
                let b: i64 = (x_target - (a * x0)) / x1;

                // Then work out the costs. It costs 0 if a or b aren't integers
                return a * COST_A + b * COST_B;
            } else {
                return 0;
            }
        } else {
            return 0;
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<(i64, i64)>> {
    // Each section is split by two sets of \r\n
    let games: Vec<&str> = input.split("\r\n\r\n").collect();

    // Parse each line into an (x, y) tuple for each game, store them in a vector
    let values: Vec<Vec<(i64, i64)>> = games
        .iter()
        .map(|&game| {
            game.split("\r\n")
                .map(|line| return get_x_y(line))
                .collect()
        })
        .collect();

    values
}

fn get_x_y(line: &str) -> (i64, i64) {
    // regex time bois
    let exp = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();

    exp.captures(line)
        .map(|caps| {
            let x = caps[1].parse().unwrap();
            let y = caps[2].parse().unwrap();
            (x, y)
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../../testInputs/day13.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 480);
    }
}
