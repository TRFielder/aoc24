use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../../inputs/day6.txt");

pub fn main() {
    println!("The result for day 6 part 1 is{}", part1(INPUT));
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

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../../testInputs/day6.txt");

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 41);
    }
}
