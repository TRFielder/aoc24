use std::collections::HashMap;

// Reads the input string as chars to a map, with x and y coordinates
pub fn read_input_to_map(input: &str) -> HashMap<(usize, usize), char> {
    let mut map = HashMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.char_indices().for_each(|(x, character)| {
            map.insert((x, y), character);
        })
    });

    return map;
}

pub fn read_input_to_imap(input: &str) -> HashMap<(isize, isize), char> {
    let mut map = HashMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.char_indices().for_each(|(x, character)| {
            map.insert((x as isize, y as isize), character);
        })
    });

    return map;
}
