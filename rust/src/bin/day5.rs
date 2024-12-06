const INPUT: &str = include_str!("../../../inputs/day5.txt");

fn main() {
    println!("The solution for day 5 part 1 is {}", part1(INPUT));
    println!("The solution for day 5 part 2 is {}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    let sections: Vec<&str> = input.split("\r\n\r\n").collect();

    // collect the rules into a vector of tuples...
    let rules: Vec<(i32, i32)> = sections[0]
        .lines()
        .map(|line| {
            let mut parts = line.split('|').map(|number| number.parse::<i32>().unwrap());

            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();

    // ...and the updates into a vector of vectors of i32
    let updates: Vec<Vec<i32>> = sections[1]
        .lines()
        .map(|line| {
            line.split(",")
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    // Iterate over each update, see if the rules have been obeyed by iterating over the vector of rules for each
    // For the ones that obey the ordering rules, add their middle value to result
    let mut result: i32 = 0;

    for update in updates {
        let mut is_in_correct_order = true;
        for (before, after) in &rules {
            let before_index = update.iter().position(|num| num == before);
            let after_index = update.iter().position(|num| num == after);

            // Only check if we found indices for both numbers
            if let (Some(before_index), Some(after_index)) = (before_index, after_index) {
                if before_index > after_index {
                    is_in_correct_order = false;
                    break;
                }
            }
        }
        if is_in_correct_order {
            result += update[update.len() / 2]
        }
    }

    result
}

// Filthy copy and paste so I can do this before leaving for work
fn part2(input: &str) -> i32 {
    let sections: Vec<&str> = input.split("\r\n\r\n").collect();

    // collect the rules into a vector of tuples...
    let rules: Vec<(i32, i32)> = sections[0]
        .lines()
        .map(|line| {
            let mut parts = line.split('|').map(|number| number.parse::<i32>().unwrap());

            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();

    // ...and the updates into a vector of vectors of i32
    let updates: Vec<Vec<i32>> = sections[1]
        .lines()
        .map(|line| {
            line.split(",")
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    // Iterate over each update, see if the rules have been obeyed by iterating over the vector of rules for each
    // For the ones that don't obey the ordering rules, push them to a new vector which we'll then sort them in

    let mut unordered_updates: Vec<Vec<i32>> = Vec::new();

    for update in updates {
        for (before, after) in &rules {
            let before_index = update.iter().position(|num| num == before);
            let after_index = update.iter().position(|num| num == after);

            // Only check if we found indices for both numbers
            if let (Some(before_index), Some(after_index)) = (before_index, after_index) {
                if before_index > after_index {
                    unordered_updates.push(update);
                    break;
                }
            }
        }
    }

    let mut result: i32 = 0;

    // Iterate through each of the unordered updates
    for mut update in unordered_updates {
        // If we make it through a loop iteration without changing anything, the list is sorted
        let mut something_changed = true;

        while something_changed {
            something_changed = false;

            for &(before, after) in &rules {
                // Find the indices of before and after
                if let (Some(before_index), Some(after_index)) = (
                    (update.iter().position(|value| value == &before)),
                    (update.iter().position(|value| value == &after)),
                ) {
                    if before_index > after_index {
                        // Swap the numbers because the rules have been broken
                        update.swap(before_index, after_index);

                        // a change has been made so we need to go again
                        something_changed = true;
                    }
                }
            }
        }

        // If we get here, the list has been sorted according to the rules... slowly
        // Add the middle element to result
        result += update[update.len() / 2];
    }

    result
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("../../../testInputs/day5.txt");
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 123);
    }
}
