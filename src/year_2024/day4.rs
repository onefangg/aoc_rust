use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn solve_part_a(input_path: &str) -> usize {
    let content = fs::read_to_string(&Path::new(input_path)).unwrap();
    // for another day, thanks to the reference on just simplifying it to 1-D vec in rust:
    // https://github.com/nbanman/pdx-puzzles/blob/main/rust/advent/src/bin/2024/y24d04.rs
    let width = content.find("\r\n").unwrap() as isize;

    let data = content
        .lines()
        .flat_map(|x| x.chars())
        .collect::<Vec<char>>();
    let x_indices = data
        .iter()
        .enumerate()
        .filter(|(_, &e)| e == 'X')
        .map(|(i, _)| i as isize)
        .collect::<Vec<isize>>();

    let exclude_left = [-width, width, -width + 1, 1, width + 1];
    let exclude_right = [-width - 1, -1, width - 1, -width, width];
    let exclude_right_and_left = [-width, width];
    let all_direction_offsets = [
        -width - 1,
        -1,
        width - 1,
        -width,
        width,
        -width + 1,
        1,
        width + 1,
    ];

    x_indices
        .iter()
        .map(|&idx| {
            let directions = match (idx % width < 3, (width - idx % width) <= 3) {
                (true, false) => exclude_left.to_vec(),
                (false, true) => exclude_right.to_vec(),
                (true, true) => exclude_right_and_left.to_vec(),
                (false, false) => all_direction_offsets.to_vec(),
            };
            directions
                .iter()
                .map(|dir| {
                    (1..4)
                        .map(|m| idx + (dir * m as isize))
                        .filter_map(|x| data.get(x as usize))
                        .collect::<String>()
                })
                .filter(|s| s.as_str() == "MAS")
                .count()
        })
        .sum()
}

pub fn solve_part_b(input_path: &str) -> usize {
    let content = fs::read_to_string(&Path::new(input_path)).unwrap();
    let width = content.find("\r\n").unwrap() as isize;
    let data = content
        .lines()
        .flat_map(|x| x.chars())
        .collect::<Vec<char>>();
    let a_indices = data
        .iter()
        .enumerate()
        .filter(|(_, &e)| e == 'A')
        .map(|(i, _)| i as isize)
        .collect::<Vec<isize>>();
    // first two are one segment for one diagonal
    // latter two are one segment for another diagonal
    let directions = [-width - 1, width + 1, -width + 1, width - 1];
    let compare = HashSet::from(['M', 'S']);

    a_indices
        .iter()
        .map(|&idx| {
            directions
                .iter()
                .map(|m| idx + m)
                .filter_map(|x| data.get(x as usize))
                .collect::<String>()
        })
        .filter(|x| {
            !x.contains("X")
                && (HashSet::from_iter(x.as_str()[..2].chars()) == compare)
                && (HashSet::from_iter(x.as_str()[2..].chars()) == compare)
        })
        .count()
}

#[cfg(test)]
mod day4_tests {
    use super::*;
    #[test]
    pub fn test_with_sample_data() {
        assert_eq!(solve_part_a("./inputs/day4_sample.txt"), 18);
        assert_eq!(solve_part_b("./inputs/day4b_sample.txt"), 9);
    }

    #[test]
    pub fn test_with_actual_data() {
        assert_eq!(solve_part_a("./inputs/day4.txt"), 2406);
        assert_eq!(solve_part_b("./inputs/day4.txt"), 1807);
    }
}
