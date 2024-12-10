use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

const INPUT: &str = include_str!("../../../inputs/day10.txt");

pub fn main() {
    let part1_start = Instant::now();
    let part1_result = part1(INPUT);
    let part1_duration = part1_start.elapsed();

    println!(
        "The result for day 10 part 1 is {}, it took {:?}",
        part1_result, part1_duration
    );
}

const DIRECTIONS: &[(isize, isize); 4] = &[
    // Up
    (0, 1),
    // Down
    (0, -1),
    // Right
    (1, 0),
    // Left
    (-1, 0),
];

fn read_input(input: &str) -> HashMap<(usize, usize), u32> {
    // Parse the input into a hashmap grid
    let mut grid = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, val) in line.chars().enumerate() {
            if let Some(digit) = val.to_digit(10) {
                grid.insert((x, y), digit);
            }
        }
    }

    grid
}

fn find_paths(
    x: usize,
    y: usize,
    map: &HashMap<(usize, usize), u32>,
    visited_peaks: &mut HashSet<(usize, usize)>,
) {
    // If we've found the peak, add it to the set of unique peaks and exit recursion
    if let Some(&value) = map.get(&(x, y)) {
        if value == 9 {
            visited_peaks.insert((x, y));
            return;
        }
    }

    for &(dx, dy) in DIRECTIONS {
        let next_x = x as isize + dx;
        let next_y = y as isize + dy;

        if next_x >= 0 && next_y >= 0 {
            let next_pos = (next_x as usize, next_y as usize);
            if let Some(&next_value) = map.get(&next_pos) {
                if let Some(&current_value) = map.get(&(x, y)) {
                    if next_value == current_value + 1 {
                        find_paths(next_pos.0, next_pos.1, map, visited_peaks);
                    }
                }
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let map = read_input(input);

    let mut result: usize = 0;
    // Iterate through every location in the map
    for (&(x, y), &value) in &map {
        // Only continue if we've found a trailhead
        if value != 0 {
            continue;
        }

        // Track how many unique peaks we can visit from this location
        let mut reachable_peaks: HashSet<(usize, usize)> = HashSet::new();
        // Do the pathfinding
        find_paths(x, y, &map, &mut reachable_peaks);

        result += reachable_peaks.len();
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../../testInputs/day10.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 36);
    }
}
