const INPUT: &str = include_str!("../../../inputs/day1.txt");

fn main() {
    println!("The result for day 1 part 1 is {}", part1(INPUT));
}

fn part1(input: &str) -> i32 {
    let mut first_vec: Vec<i32> = Vec::new();
    let mut second_vec: Vec<i32> = Vec::new();

    for line in input.lines() {
        // Split the line by the white space in between each number, put each number into the corresponding vector
        let mut split = line.split_whitespace();

        if let (Some(first), Some(second)) = (split.next(), split.next()) {
            first_vec.push(first.parse().unwrap());
            second_vec.push(second.parse().unwrap());
        } else {
            eprintln!("Line format is invalid: {}", line);
        }
    }

    // Sort vectors in descending order
    first_vec.sort_by(|a, b| b.cmp(a));
    second_vec.sort_by(|a, b| b.cmp(a));

    // Iterate through both vectors at once, find the absolute difference between them and sum it all up
    first_vec
        .iter()
        .zip(second_vec.iter())
        .map(|(first, second)| (first - second).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("../../../testInputs/day1.txt");
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 11);
    }
}
