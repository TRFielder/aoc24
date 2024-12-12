use aoc24::utils::parsing::read_input_to_imap;
use std::{collections::HashSet, time::Instant};

const INPUT: &str = include_str!("../../../inputs/day12.txt");

pub fn main() {
    let part1_start = Instant::now();
    let part1_result = part1(INPUT);
    let part1_duration = part1_start.elapsed();

    println!(
        "The result for day 12 part 1 is {}, it took {:?}",
        part1_result, part1_duration
    );

    let part2_start = Instant::now();
    let part2_result = part2(INPUT);
    let part2_duration = part2_start.elapsed();

    println!(
        "The result for day 12 part 2 is {}, it took {:?}",
        part2_result, part2_duration
    );
}

fn part1(input: &str) -> isize {
    let map = read_input_to_imap(input);

    let mut result: isize = 0;

    // Locations we've already been through
    let mut checked_locations: HashSet<(isize, isize)> = HashSet::new();

    // Work through the whole map and add locations to the set
    for (position, plant) in map.iter() {
        // Skip if we've already processed this position
        if checked_locations.contains(position) {
            continue;
        }

        // We need a set to keep track of the shape of each connected region
        let mut shape = HashSet::new();
        // Add the initial position to the shape
        shape.insert(*position);

        // Push all the neighbouring coordinates into a vector of locations we need to check
        // To see if they are part of the shape as well
        let mut neighbours_to_check = Vec::new();

        // Get neighbouring coordinates of the position, filter out the ones that we've already processed
        get_neighbouring_coordinates(position)
            .into_iter()
            .filter(|neighbour| !checked_locations.contains(neighbour))
            .for_each(|neighbour| neighbours_to_check.push(neighbour));

        // Work through each neighbour
        while let Some(neighbour) = neighbours_to_check.pop() {
            // Skip this neighbour if it's already been checked on another loop iteration
            if checked_locations.contains(&neighbour) {
                continue;
            }

            // If the value matches the current plant, it's part of the shape
            if map.get(&neighbour) == Some(plant) {
                // so mark it checked...
                checked_locations.insert(neighbour);
                // and put it in the shape
                shape.insert(neighbour);

                // Add the neighbours to the list of positions to check
                get_neighbouring_coordinates(&neighbour)
                    .into_iter()
                    // But only the ones that we haven't already checked
                    .filter(|&pos| !checked_locations.contains(&pos))
                    .for_each(|pos| neighbours_to_check.push(pos))
            }
        }

        // Once we get here, we should have all the connected positions in a single region
        result += calculate_perimeter(&shape) * shape.len() as isize;
    }

    result
}

// Literally the same as part 1 except with edges instead of perimeter
fn part2(input: &str) -> isize {
    let map = read_input_to_imap(input);

    let mut result: isize = 0;

    // Locations we've already been through
    let mut checked_locations: HashSet<(isize, isize)> = HashSet::new();

    // Work through the whole map and add locations to the set
    for (position, plant) in map.iter() {
        // Skip if we've already processed this position
        if checked_locations.contains(position) {
            continue;
        }

        // We need a set to keep track of the shape of each connected region
        let mut shape = HashSet::new();
        // Add the initial position to the shape
        shape.insert(*position);

        // Push all the neighbouring coordinates into a vector of locations we need to check
        // To see if they are part of the shape as well
        let mut neighbours_to_check = Vec::new();

        // Get neighbouring coordinates of the position, filter out the ones that we've already processed
        get_neighbouring_coordinates(position)
            .into_iter()
            .filter(|neighbour| !checked_locations.contains(neighbour))
            .for_each(|neighbour| neighbours_to_check.push(neighbour));

        // Work through each neighbour
        while let Some(neighbour) = neighbours_to_check.pop() {
            // Skip this neighbour if it's already been checked on another loop iteration
            if checked_locations.contains(&neighbour) {
                continue;
            }

            // If the value matches the current plant, it's part of the shape
            if map.get(&neighbour) == Some(plant) {
                // so mark it checked...
                checked_locations.insert(neighbour);
                // and put it in the shape
                shape.insert(neighbour);

                // Add the neighbours to the list of positions to check
                get_neighbouring_coordinates(&neighbour)
                    .into_iter()
                    // But only the ones that we haven't already checked
                    .filter(|&pos| !checked_locations.contains(&pos))
                    .for_each(|pos| neighbours_to_check.push(pos))
            }
        }

        // Once we get here, we should have all the connected positions in a single region
        result += calculate_sides(&shape) * shape.len() as isize;
    }

    result
}

// Coordinates are (x, y) tuples, we use isize so we can handle edges (because the edge is still a perimeter but could have an invalid usize index of -1)
fn get_neighbouring_coordinates(position: &(isize, isize)) -> Vec<(isize, isize)> {
    let mut neighbours = Vec::new();

    neighbours.push((position.0 - 1, position.1));
    neighbours.push((position.0, position.1 - 1));

    neighbours.push((position.0 + 1, position.1));
    neighbours.push((position.0, position.1 + 1));

    neighbours
}

// The perimeter is the same length as the set of all immediately neighbouring
// coordinates that aren't in the actual region
fn calculate_perimeter(region: &HashSet<(isize, isize)>) -> isize {
    let mut total = 0;
    for position in region.iter() {
        // Get neighbouring coordinates for each position in the region
        total += get_neighbouring_coordinates(position)
            .into_iter()
            // Take only the coordinates that aren't already part of the shape
            .filter(|neighbour| !region.contains(neighbour))
            // and just count them
            .count() as isize
    }

    total
}

// A shape has the same number of sides as it does corners, so we just need to count corners
fn calculate_sides(shape: &HashSet<(isize, isize)>) -> isize {
    let mut total = 0;

    for position in shape.iter() {
        // Count how many neighbours aren't part of the shape
        let count = get_neighbouring_coordinates(position)
            .into_iter()
            .filter(|neighbour| !shape.contains(neighbour))
            .count();

        // If the shape is a single cell, it has 4 corners and therefore 4 sides
        if count == 4 {
            total += 4;
        }

        // If the cell has 3 sides clear, it adds 2 corners
        if count == 3 {
            total += 2;
        }

        // If the cell has 2 sides clear, those sides need to be joined otherwise it adds no corners
        // Checck if the shape has neighbours on the north and south
        if count == 2 {
            // Check if the shape is connected vertically or horizontally
            if shape.contains(&(position.0, position.1 + 1))
                && shape.contains(&(position.0, position.1 - 1))
                || shape.contains(&(position.0 + 1, position.1))
                    && shape.contains(&(position.0 - 1, position.1))
            {
                // If so, there are no corners on this position
                total += 0;
            } else {
                // Otherwise, there are
                total += 1;
            }
        }

        // Can also have corners like on the inside of an L, if the diagonal is missing but the corresponding vertical + horizontal aren't

        // Northeast diagonal
        if !shape.contains(&(position.0 + 1, position.1 + 1))
            && shape.contains(&(position.0 + 1, position.1))
            && shape.contains(&(position.0, position.1 + 1))
        {
            total += 1;
        }

        // Southeast diagonal
        if !shape.contains(&(position.0 + 1, position.1 - 1))
            && shape.contains(&(position.0 + 1, position.1))
            && shape.contains(&(position.0, position.1 - 1))
        {
            total += 1;
        }

        // Southwest diagonal
        if !shape.contains(&(position.0 - 1, position.1 - 1))
            && shape.contains(&(position.0 - 1, position.1))
            && shape.contains(&(position.0, position.1 - 1))
        {
            total += 1;
        }

        // Northwest diagonal
        if !shape.contains(&(position.0 - 1, position.1 + 1))
            && shape.contains(&(position.0 - 1, position.1))
            && shape.contains(&(position.0, position.1 + 1))
        {
            total += 1;
        }
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_1: &str = include_str!("../../../testInputs/day12_1.txt");
    const TEST_INPUT_2: &str = include_str!("../../../testInputs/day12_2.txt");
    const TEST_INPUT_3: &str = include_str!("../../../testInputs/day12_3.txt");
    const TEST_INPUT_4: &str = include_str!("../../../testInputs/day12_4.txt");
    const TEST_INPUT_5: &str = include_str!("../../../testInputs/day12_5.txt");

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(TEST_INPUT_1), 140);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(TEST_INPUT_2), 772);
    }

    #[test]
    fn test_part1_3() {
        assert_eq!(part1(TEST_INPUT_3), 1930);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(part2(TEST_INPUT_1), 80);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2(TEST_INPUT_4), 236);
    }

    #[test]
    fn test_part2_3() {
        assert_eq!(part2(TEST_INPUT_5), 368);
    }
}
