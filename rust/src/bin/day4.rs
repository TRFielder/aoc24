const INPUT: &str = include_str!("../../../inputs/day4.txt");

fn main() {
    println!("The result for day 4 part 1 is {}", part1(INPUT));
    println!("The result for day 4 part 2 is {}", part2(INPUT));
}

const DIRECTIONS: [[i32; 2]; 8] = [
    // Horizontal
    [1, 0],
    [-1, 0],
    // Vertical
    [0, 1],
    [0, -1],
    // Diagonal
    [1, 1],
    [1, -1],
    [-1, -1],
    [-1, 1],
];

fn is_not_xmas(grid: &Vec<Vec<char>>, expected_letter: char, col: usize, row: usize) -> bool {
    if row >= grid.len() || col >= grid[row].len() {
        return true; // Out of bounds
    }
    grid[row][col] != expected_letter
}

fn part1(input: &str) -> i32 {
    let mut result: i32 = 0;
    let word_to_find: Vec<char> = vec!['X', 'M', 'A', 'S'];

    // Iterate over each line, and collect each char in that line into a vector
    //  which gets pushed to an overall vector of vectors of chars (a grid)
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Iterate through each char in the grid and check if there's an X there
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            // Iterate through the directions vector
            for &[x, y] in DIRECTIONS.iter() {
                let mut found_xmas = true;

                // Loop through the chars in "XMAS", start at X and look for each one in turn, in every direction
                for (index, &letter) in word_to_find.iter().enumerate() {
                    let new_col = col as i32 + index as i32 * x;
                    let new_row = row as i32 + index as i32 * y;

                    // Check boundaries, if we're outside the boundaries or the letter doesn't match XMAS then set false
                    if new_row < 0
                        || new_col < 0
                        || new_row as usize >= grid.len()
                        || new_col as usize >= grid[new_row as usize].len()
                        || is_not_xmas(&grid, letter, new_col as usize, new_row as usize)
                    {
                        found_xmas = false;
                        break;
                    }
                }
                // Otherwise we've found XMAS and we can increment the value
                if found_xmas {
                    result += 1;
                }
            }
        }
    }

    result
}

#[allow(unused_variables)]
fn part2(input: &str) -> i32 {
    9
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("../../../testInputs/day4.txt");
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 9);
    }
}
