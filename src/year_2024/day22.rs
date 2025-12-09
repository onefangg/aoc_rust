use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input_seeds: Vec<i64>) -> (i64, i64) {
    let mut p1 = 0i64;
    let mut p2_lookup = HashMap::new();

    for seed in input_seeds {
        let mut temp = seed;
        let mut prices = [0i64; 2000];
        let mut patterns = HashSet::new();

        for iter in 0..2000 {
            temp = generate_pseudo_random(temp as u64) as i64;
            prices[iter] = temp % 10;
        }
        p1 = p1 + temp;

        for (a, b, c, d, e) in prices.iter().tuple_windows() {
            let key = (b - a, c - b, d - c, e - d);
            if patterns.insert(key) {
                *p2_lookup.entry(key).or_default() += e;
            }
        }
    }
    (p1, *p2_lookup.values().max().unwrap())
}

pub fn generate_pseudo_random(seed: u64) -> u64 {
    let a = ((seed * 64) ^ seed) % 16777216;
    let b = ((a / 32) ^ a) % 16777216;
    let c = ((b * 2048) ^ b) % 16777216;
    c
}

#[cfg(test)]
mod day22_tests {
    use super::*;
    use std::fs;
    #[test]
    pub fn test_with_sample_data() {
        let (p1, _) = solve(vec![1, 10, 100, 2024]);
        assert_eq!(p1, 37327623);

        let (_, p2) = solve(vec![1, 2, 3, 2024]);
        assert_eq!(p2, 23);
    }
    #[test]
    pub fn test_with_actual_data() {
        let read_input = fs::read_to_string("../../inputs/day22.txt")
            .unwrap()
            .lines()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let (p1, p2) = solve(read_input);
        assert_eq!(p1, 19458130434);
        assert_eq!(p2, 2130);
    }
}
