use aoc24::utils::parsing::read_input_to_map;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

const INPUT: &str = include_str!("../../../inputs/day8.txt");

pub fn main() {
    // Read the input to a hashmap
    let map = read_input_to_map(INPUT);

    let part1_start = Instant::now();
    let part1_result = part1(&map);
    let part1_duration = part1_start.elapsed();

    println!(
        "The result for day 8 part 1 is {}, it took {:?}",
        part1_result, part1_duration
    );
}

fn part1(input: &HashMap<(usize, usize), char>) -> usize {
    // We need to find all the unique antennas (characters) in the map
    let antenna_types: HashSet<char> = input
        .values()
        .copied()
        .filter(|&char| char != '.')
        .collect();

    // Get the max x and y values
    let rows = input.keys().map(|&(_, y)| y).max().unwrap() + 1;
    let cols = input.keys().map(|&(x, _)| x).max().unwrap() + 1;

    // Get all positions of each antenna type, and group them
    let mut antenna_positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (&position, &character) in input {
        if character != '.' {
            // Add the position to the map of only antennas
            antenna_positions
                .entry(character)
                .or_default()
                .push(position);
        }
    }

    let mut antinode_positions: HashSet<(usize, usize)> = HashSet::new();
    // Calculate vectors between pairs of antennas and work out antinodes from that
    for &antenna in &antenna_types {
        if let Some(positions) = antenna_positions.get(&antenna) {
            if positions.len() < 2 {
                // There are no antinodes if there are no pairs
                continue;
            }

            // Iterate through all possible pairs of antennas
            for i in 0..positions.len() {
                for j in i + 1..positions.len() {
                    let (x1, y1) = positions[i];
                    let (x2, y2) = positions[j];

                    // Calculate vector between the pair. Can't be usize for this to work
                    let dx = x2 as isize - x1 as isize;
                    let dy = y2 as isize - y1 as isize;

                    // Get positions in both directions from both antennas
                    for direction in [-1, 1] {
                        let antinode_x1 = x1 as isize + (direction * dx);
                        let antinode_y1 = y1 as isize + (direction * dy);

                        let antinode_x2 = x2 as isize + (direction * dx);
                        let antinode_y2 = y2 as isize + (direction * dy);

                        let pos1 = (antinode_x1 as usize, antinode_y1 as usize);
                        let pos2 = (antinode_x2 as usize, antinode_y2 as usize);

                        // If the antinodes generated don't overlap the antenna pair, put it in the Set
                        // (I expect one will overlap each time but that's ok)
                        if pos1.0 < cols && pos1.1 < rows && !positions.contains(&pos1) {
                            antinode_positions.insert(pos1);
                        }

                        if pos2.0 < cols && pos2.1 < rows && !positions.contains(&pos2) {
                            antinode_positions.insert(pos2);
                        }
                    }
                }
            }
        }
    }

    antinode_positions.len()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../../testInputs/day8.txt");

    #[test]
    fn test_part1() {
        let map = read_input_to_map(TEST_INPUT);
        assert_eq!(part1(&map), 14);
    }
}
