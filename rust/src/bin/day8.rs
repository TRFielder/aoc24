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
    let part1_result = solve(&map, false);
    let part1_duration = part1_start.elapsed();

    println!(
        "The result for day 8 part 1 is {}, it took {:?}",
        part1_result, part1_duration
    );

    let part2_start = Instant::now();
    let part2_result = solve(&map, true);
    let part2_duration = part2_start.elapsed();

    println!(
        "The result for day 8 part 2 is {}, it took {:?}",
        part2_result, part2_duration
    );
}

fn solve(input: &HashMap<(usize, usize), char>, with_multiple: bool) -> usize {
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
                        // We can go multiples of the vector so long as at least one is within the bounds of the map
                        let mut multiple = 1;

                        loop {
                            let antinode_x1 = x1 as isize + (direction * multiple * dx);
                            let antinode_y1 = y1 as isize + (direction * multiple * dy);

                            let antinode_x2 = x2 as isize + (direction * multiple * dx);
                            let antinode_y2 = y2 as isize + (direction * multiple * dy);

                            let pos1 = (antinode_x1 as usize, antinode_y1 as usize);
                            let pos2 = (antinode_x2 as usize, antinode_y2 as usize);

                            // Break the loop if no antinode is within the map
                            if (antinode_x1 < 0
                                || antinode_y1 < 0
                                || antinode_x1 >= cols as isize
                                || antinode_y1 >= rows as isize)
                                && (antinode_x2 < 0
                                    || antinode_y2 < 0
                                    || antinode_x2 >= cols as isize
                                    || antinode_y2 >= rows as isize)
                            {
                                break;
                            }

                            // If the antinodes generated don't overlap the antenna pair, put it in the Set
                            // (I expect one will overlap each time but that's ok)
                            // If we're allowing resonance (part 2) allow overlap
                            if pos1.0 < cols && pos1.1 < rows {
                                if with_multiple {
                                    antinode_positions.insert(pos1);
                                } else if !with_multiple && !positions.contains(&pos1) {
                                    antinode_positions.insert(pos1);
                                }
                            }

                            if pos2.0 < cols && pos2.1 < rows {
                                if with_multiple {
                                    antinode_positions.insert(pos2);
                                } else if !with_multiple && !positions.contains(&pos2) {
                                    antinode_positions.insert(pos2);
                                }
                            }

                            // Add loop breaker so part 1 still works
                            if !with_multiple {
                                break;
                            }
                            // Increment the multiplier for the next loop
                            multiple += 1;
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
        assert_eq!(solve(&map, false), 14);
    }

    #[test]
    fn test_part2() {
        let map = read_input_to_map(TEST_INPUT);
        assert_eq!(solve(&map, true), 34);
    }
}
