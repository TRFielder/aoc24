const INPUT: &str = include_str!("../../../inputs/day1.txt");

fn main() {
    println!("The result for day 1 part 1 is {}", part1(INPUT));
    println!("The result for day 1 part 2 is {}", part2(INPUT));
}

fn read_input_to_vectors(input: &str) -> (Vec<i32>, Vec<i32>) {
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

    return (first_vec, second_vec);
}

fn part1(input: &str) -> i32 {
    let (mut first_vec, mut second_vec) = read_input_to_vectors(input);

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

fn part2(input: &str) -> i32 {
    let (first_vec, second_vec) = read_input_to_vectors(input);

    // Don't need to sort the vectors this time
    // (or modify them at all, so we can avoid the 'mut' keyword)
    // for each item in first_vec, count how many times it appears in second_vec
    // multiply that item by the count, and add the result to the total

    let mut result: i32 = 0;

    for item in first_vec {
        let count = second_vec.iter().filter(|&&value| value == item).count() as i32;
        result += item * count;
    }
    result
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("../../../testInputs/day1.txt");
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 31);
    }
}
