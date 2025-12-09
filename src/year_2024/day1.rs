use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn parse_puzzle_input(input_path: &str) -> (Vec<u32>, Vec<u32>) {
    let content = fs::read_to_string(&Path::new(input_path)).unwrap();

    content
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect_tuple::<(u32, u32)>()
                .unwrap()
        })
        .unzip()
}

pub fn solve_part_a(input_path: &str) -> u32 {
    let (left, right) = parse_puzzle_input(input_path);
    let sorted_right: Vec<u32> = right.into_iter().sorted().collect();
    left.iter()
        .sorted()
        .enumerate()
        .map(|(idx, e)| e.abs_diff(sorted_right[idx]))
        .sum::<u32>()
}

pub fn solve_part_b(input_path: &str) -> u32 {
    let (left, right) = parse_puzzle_input(input_path);

    // alternatively let itertools handle it for us
    // let right_freq = right.into_iter().counts();
    let right_freq = right.iter().copied().fold(HashMap::new(), |mut acc, e| {
        acc.entry(e).and_modify(|frq| *frq += 1).or_insert(1);
        acc
    });
    left.iter()
        .map(|x| x * right_freq.get(x).unwrap_or(&0))
        .sum::<u32>()
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    pub fn test_with_sample_data() {
        assert_eq!(solve_part_a("./inputs/day1_sample.txt"), 11);
        assert_eq!(solve_part_b("./inputs/day1_sample.txt"), 31);
    }

    #[test]
    pub fn test_with_actual_data() {
        assert_eq!(solve_part_a("./inputs/day1.txt"), 1941353);
        assert_eq!(solve_part_b("./inputs/day1.txt"), 22539317);
    }
}
