use regex::Regex;
use std::fs;
use std::path::Path;

pub fn solve_part_a(input_path: &str) -> u32 {
    let content = fs::read_to_string(&Path::new(input_path)).unwrap();
    let pattern = Regex::new(r"mul\((\d+)\,(\d+)\)").unwrap();
    pattern
        .captures_iter(&content)
        .map(|x| {
            let (_, [l, r]) = x.extract();
            l.parse::<u32>().unwrap() * r.parse::<u32>().unwrap()
        })
        .sum::<u32>()
}

pub fn solve_part_b(input_path: &str) -> u32 {
    let content = fs::read_to_string(&Path::new(input_path)).unwrap();
    let pattern = Regex::new(r"mul\((\d+)\,(\d+)\)|do\(\)|don't\(\)").unwrap();
    // fold logic referenced from: https://github.com/nbanman/pdx-puzzles/blob/main/rust/advent/src/bin/2024/y24d03.rs#L26-L42
    pattern
        .captures_iter(&content)
        .fold((0, true), |(res, is_enabled), cap| {
            if is_enabled && cap.get(0).unwrap().as_str().starts_with("mul") {
                let pdt = cap.get(1).unwrap().as_str().parse::<u32>().unwrap()
                    * cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
                return (res + pdt, true);
            } else {
                match cap.get(0).unwrap().as_str() {
                    "do()" => (res, true),
                    "don't()" => (res, false),
                    _ => (res, is_enabled),
                }
            }
        })
        .0
}

#[cfg(test)]
mod day3_tests {
    use super::*;
    #[test]
    pub fn test_with_sample_data() {
        assert_eq!(solve_part_a("./inputs/day3a_sample.txt"), 161);
        assert_eq!(solve_part_b("./inputs/day3b_sample.txt"), 48);
    }

    #[test]
    pub fn test_with_actual_data() {
        assert_eq!(solve_part_a("./inputs/day3.txt"), 178886550);
        assert_eq!(solve_part_b("./inputs/day3.txt"), 87163705);
    }
}
