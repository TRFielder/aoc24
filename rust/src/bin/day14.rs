use image::{GrayImage, Luma};
use regex::Regex;
use std::{fs, path::Path, time::Instant};

const INPUT: &str = include_str!("../../../inputs/day14.txt");

pub fn main() {
    let part1_start = Instant::now();
    let part1_result = part1(INPUT, 101, 103);
    let part1_duration = part1_start.elapsed();

    println!(
        "The result for day 14 part 1 is {}, it took {:?}",
        part1_result, part1_duration
    );

    let part2_start = Instant::now();
    part2(INPUT, 101, 103);
    let part2_duration = part2_start.elapsed();

    println!(
        "The result for day 14 part 2 took {:?}. Good luck finding the right image!",
        part2_duration
    );
}

fn part1(input: &str, x_max: i32, y_max: i32) -> i32 {
    let exp = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    // Get a vector of tuples of all the info for a given robot. x,y position first, then x,y velocity
    let mut robots: Vec<((i32, i32), (i32, i32))> = Vec::new();

    for line in input.lines() {
        if let Some(capture) = exp.captures(line) {
            robots.push((
                (
                    capture[1].parse::<i32>().unwrap(),
                    capture[2].parse::<i32>().unwrap(),
                ),
                (
                    capture[3].parse::<i32>().unwrap(),
                    capture[4].parse::<i32>().unwrap(),
                ),
            ));
        }
    }

    // We need to simulate the robots for 100 seconds of movement
    let robots = move_robots(robots, x_max, y_max, 100);

    // Now just count the robots in each quadrant...
    count_robots(robots, x_max, y_max)
}

// Copy part 1's code but with a twist...
fn part2(input: &str, x_max: i32, y_max: i32) {
    let exp = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    // Get a vector of tuples of all the info for a given robot. x,y position first, then x,y velocity
    let mut robots: Vec<((i32, i32), (i32, i32))> = Vec::new();

    for line in input.lines() {
        if let Some(capture) = exp.captures(line) {
            robots.push((
                (
                    capture[1].parse::<i32>().unwrap(),
                    capture[2].parse::<i32>().unwrap(),
                ),
                (
                    capture[3].parse::<i32>().unwrap(),
                    capture[4].parse::<i32>().unwrap(),
                ),
            ));
        }
    }

    // Simulate for a 10000 seconds of movement and save an image for each second.
    // This is gonna be fun...
    move_robots_and_save_image(robots, x_max, y_max, 10000);
}

// Returns the robots' positions as a vector after duration seconds of movement
fn move_robots(
    robots: Vec<((i32, i32), (i32, i32))>,
    x_max: i32,
    y_max: i32,
    duration: i32,
) -> Vec<(i32, i32)> {
    robots
        .iter()
        .map(|((x, y), (dx, dy))| {
            (
                // position = velocity * time
                // Thanks rust for having handy remainder functions for this exact problem...
                (x + dx * duration).rem_euclid(x_max),
                (y + dy * duration).rem_euclid(y_max),
            )
        })
        .collect()
}

// Returns the robots' positions as a vector after duration seconds of movement, and saves an image of the positions
fn move_robots_and_save_image(
    robots: Vec<((i32, i32), (i32, i32))>,
    x_max: i32,
    y_max: i32,
    duration: i32,
) {
    let output_dir = "./day14img";
    fs::create_dir_all(output_dir).unwrap();
    for i in 1..duration {
        let new_positions: Vec<(i32, i32)> = robots
            .iter()
            .map(|((x, y), (dx, dy))| {
                (
                    // position = velocity * time
                    // Thanks rust for having handy remainder functions for this exact problem...
                    (x + dx * i).rem_euclid(x_max),
                    (y + dy * i).rem_euclid(y_max),
                )
            })
            .collect();

        // Print every robot's position to the image grid
        let mut img = GrayImage::new(x_max as u32, y_max as u32);
        for (x, y) in new_positions.iter() {
            img.put_pixel(*x as u32, *y as u32, Luma([255]));
        }
        let file_path = format!("{}/{}.png", output_dir, i);
        img.save(Path::new(&file_path)).unwrap();
    }
}

fn count_robots(robots: Vec<(i32, i32)>, x_max: i32, y_max: i32) -> i32 {
    // Q 1 is top left, assigned clockwise from there
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for (x, y) in robots {
        if x < x_max / 2 && y < y_max / 2 {
            q1 += 1;
        }
        if x > x_max / 2 && y < y_max / 2 {
            q2 += 1;
        }
        if x > x_max / 2 && y > y_max / 2 {
            q3 += 1;
        }
        if x < x_max / 2 && y > y_max / 2 {
            q4 += 1;
        }
    }

    // Safety score is all of these multiplied together
    q1 * q2 * q3 * q4
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../../testInputs/day14.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT, 11, 7), 12);
    }
}
