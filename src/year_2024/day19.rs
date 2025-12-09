use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

pub fn recurse_through<'a>(
    design: &'a str,
    avail_designs: &HashSet<&str>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if cache.contains_key(design) {
        return cache[design];
    }
    if design.is_empty() {
        return 1;
    }
    let mut sum = 0;
    for a in avail_designs {
        if a.len() > design.len() {
            continue;
        }
        if design.starts_with(a) {
            sum += recurse_through(&design[a.len()..], avail_designs, cache);
        }
    }
    cache.insert(design, sum);
    sum
}

pub fn solve(input_str: &str) -> (usize, usize) {
    let content = fs::read_to_string(Path::new(input_str)).unwrap();
    let (parse_towels, parse_designs) = content.split_once("\r\n\r\n").unwrap();
    let towels = parse_towels.split(", ").collect::<HashSet<&str>>();
    let designs = parse_designs.lines().collect::<Vec<&str>>();
    let mut cache = HashMap::new();

    let mut p1 = 0;
    let mut p2 = 0;
    for d in designs {
        let res = recurse_through(d, &towels, &mut cache);
        if res > 0 {
            p2 += res;
            p1 += 1;
        }
    }
    (p1, p2)
}

#[cfg(test)]
mod day19_tests {
    use super::*;
    #[test]
    pub fn test_with_sample_data() {
        let (p1, p2) = solve("./inputs/day19_sample.txt");
        assert_eq!(p1, 6);
        assert_eq!(p2, 16);
    }
    #[test]
    pub fn test_with_actual_data() {
        let (p1, p2) = solve("./inputs/day19.txt");
        assert_eq!(p1, 278);
        assert_eq!(p2, 569808947758890);
    }
}
