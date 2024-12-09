use std::time::Instant;

const INPUT: &str = include_str!("../../../inputs/day9.txt");

pub fn main() {
    let part1_start = Instant::now();
    let part1_result = part1(INPUT);
    let part1_duration = part1_start.elapsed();

    println!(
        "The result for day 9 part 1 is {}, it took {:?}",
        part1_result, part1_duration
    );
}

fn part1(input: &str) -> i64 {
    // 0th index tells us file size, 1st index tells us free space size
    // eg [1, 2] is 1 file, 2 free space
    let split: Vec<i64> = input
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .map(|digit| digit as i64)
        .collect();

    // Build the block layout from here
    // the numbers alternate between being files and free space, so ones with index % 2 == 0 are files
    // and the index / 2 is their ID if that's the case
    // if it's a file, loop up to the file size adding the ID to the vector
    // if it's free space, loop up to the space size adding "None" to the vector"
    let mut blocks: Vec<Option<i64>> = split
        .iter()
        .enumerate()
        .flat_map(|(index, &value)| {
            // If it's a file

            if index % 2 == 0 {
                let file_id = (index / 2) as i64;
                // Create a vector of "value" length containing file_id
                return vec![Some(file_id); value as usize];
            } else {
                // Create a vector of "value" length containing None
                return vec![None; value as usize];
            }
        })
        .collect();

    // Grab element from the end of blocks and swap it with the first "None"
    let mut last_some_index = blocks.len() - 1;
    let mut first_none_index = 0;

    // While there is a None value before a Some value in the vector...
    while first_none_index < last_some_index {
        // Find the last Some index
        while last_some_index > 0 && blocks[last_some_index].is_none() {
            last_some_index -= 1
        }

        // Find the first None index
        while first_none_index < blocks.len() && blocks[first_none_index].is_some() {
            first_none_index += 1;
        }

        // if we have a Some and a None, swap them
        if first_none_index < blocks.len() && last_some_index > first_none_index {
            blocks.swap(first_none_index, last_some_index)
        } else {
            break;
        }
    }

    // turn it into a list of just the data file IDs, repeated based on their size
    let just_data: Vec<i64> = blocks.into_iter().filter_map(|value| value).collect();

    // Multiply the file ID by its position to get the checksum of the whole thing
    just_data
        .iter()
        .enumerate()
        .fold(0, |acc, (index, value)| acc + (index as i64 * value))
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../../testInputs/day9.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1928);
    }
}
