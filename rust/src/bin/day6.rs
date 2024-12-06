use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

const INPUT: &str = include_str!("../../../inputs/day6.txt");

pub fn main() {
    let part1_start = Instant::now();
    let part1_result = part1(INPUT);
    let part1_duration = part1_start.elapsed();

    let part2_start = Instant::now();
    let part2_result = part2(INPUT);
    let part2_duration = part2_start.elapsed();

    println!(
        "The result for day 6 part 1 is {}, it took {:?}",
        part1_result, part1_duration
    );
    println!(
        "The result for day 6 part 2 is {}, it took {:?}",
        part2_result, part2_duration
    );
}

fn part1(input: &str) -> i32 {
    // Let's try this with a HashMap because clearly vectors aren't the one today
    let mut map = HashMap::new();
    let mut current_position = None;

    // Let's find the guard first
    // iterate through each possible y value (row)
    input.lines().enumerate().for_each(|(y, line)| {
        // And then each possible x value (column) in that row
        line.char_indices().for_each(|(x, character)| {
            let x = x as i32;
            let y = y as i32;

            // It's a ^ at the start in my input
            if character == '^' {
                // Set the current_position to the (x, y) value where the ^ was found
                current_position = Some((x, y));
                // Change that original location to a "."
                map.insert((x, y), '.');
            } else {
                // Don't change other characters, just insert them into the map
                map.insert((x, y), character);
            }
        });
    });

    // Change the guard's position
    let mut position = current_position.unwrap();
    let mut direction = '^';

    // Let's create a list of all the (x, y) positions he's visited. HashSet so we don't end up with duplicate values at the end!
    let mut visited_locations = HashSet::new();

    // map.contains_key will not run if the guard's position isn't in the map - ie they've left
    while map.contains_key(&position) {
        let (x, y) = position;
        // Add the guard's current location to the list
        visited_locations.insert(position);

        // Work out the next position the guard should move to based on their facing
        let next = match direction {
            // Counterintuitive but y-1 is actually up, because lower index in the map
            '^' => (x, y - 1),
            'v' => (x, y + 1),
            '<' => (x - 1, y),
            '>' => (x + 1, y),
            // We should never get here... try something new cos can't return None
            _ => unreachable!(),
        };

        // Check the next position, see if it's a blocker. If so, turn the guard to the right
        if let Some('#') = map.get(&next) {
            // println!("bump at {x},{y}");
            direction = match direction {
                '^' => '>',
                'v' => '<',
                '<' => '^',
                '>' => 'v',
                // We should never get here... but I can't return None
                _ => unreachable!(),
            };
        } else {
            // If the guard isn't blocked, update their position
            position = next;
        }
    }
    // Return the number of (unique) elements in the HashSet
    visited_locations.len() as i32
}

fn part2(input: &str) -> i32 {
    // Copying an awful lot of part 1 into this
    let mut map = HashMap::new();
    let mut current_position = None;
    let mut open_spaces = HashSet::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.char_indices().for_each(|(x, character)| {
            let x = x as i32;
            let y = y as i32;
            // Get the guard's position
            if character == '^' {
                // Set the current_position to the (x, y) value where the ^ was found
                current_position = Some((x, y));
                // Put a "." in that location in the map
                map.insert((x, y), '.');
            } else {
                // Don't touch the original cahracter otherwise
                map.insert((x, y), character);
            }
            if character == '.' {
                // Add all "." (x, y) values to a Set of open spaces that we can fill with "#"
                open_spaces.insert((x, y));
            }
        });
    });

    let position = current_position.unwrap();
    // Starting direction is up in my input
    let direction = '^';

    // For every possible open space, try inserting a "#" and see if it makes an infinite loop..
    open_spaces
        .into_iter()
        // Filter by whether it makes an infinite loop
        .filter(|open_space| {
            // Clone the map so we're not modifying the OG and filling the whole thing with "#"
            let mut cloned_map = map.clone();

            // Try putting a "#" at this open space, and see if it creates a loop
            cloned_map.insert(*open_space, '#');
            is_infinite_loop(cloned_map, position, direction)
        })
        // Count how many spaces there are that make an infinite loop when "#"ified
        .count() as i32
}

fn is_infinite_loop(
    map: HashMap<(i32, i32), char>,
    mut position: (i32, i32),
    mut direction: char,
) -> bool {
    // Another HashSet of visited spaces
    let mut visited = HashSet::new();

    // Whilst the guard is on the map...
    while map.contains_key(&position) {
        let (x, y) = position;
        // If we've been at this position before, facing this way, then we're in an infinite loop!
        if visited.contains(&(position, direction)) {
            return true;
        }

        // Insert our position and direction to the HashSet
        visited.insert((position, direction));

        // Work out the next position we're moving to
        let next_position = match direction {
            '^' => (x, y - 1),
            'v' => (x, y + 1),
            '<' => (x - 1, y),
            '>' => (x + 1, y),
            _ => unreachable!(),
        };

        // If the next position is a "#" then turn the guard to the right
        if let Some('#') = map.get(&next_position) {
            direction = match direction {
                '^' => '>',
                'v' => '<',
                '<' => '^',
                '>' => 'v',
                _ => unreachable!(),
            };
        } else {
            // Otherwise move them to the next position and go back to the start of the loop
            position = next_position;
        }
    }
    // Not an infinite loop if we get here, because the guard makes it off the map
    false
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../../testInputs/day6.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 6);
    }
}
