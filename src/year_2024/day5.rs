use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

pub fn solve_part_a_and_b(input_path: &str) -> (usize, usize) {
    let content = fs::read_to_string(&Path::new(input_path)).unwrap();
    let parsed_content = content.split("\r\n\r\n").collect::<Vec<&str>>();
    let (rules, updates) = (parsed_content[0], parsed_content[1]);

    let rules_lookup = rules
        .lines()
        .map(|r| {
            let (l, r) = r.split_once('|').unwrap();
            (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap())
        })
        .fold(
            HashMap::<usize, HashSet<usize>>::new(),
            |mut acc, (l, r)| {
                acc.entry(l)
                    .and_modify(|e| {
                        (*e).insert(r);
                    })
                    .or_insert(HashSet::from([r]));
                acc
            },
        );

    let mut part_a = 0usize;
    let mut part_b = 0usize;
    updates
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect_vec()
        })
        .for_each(|x| {
            if x.iter()
                .is_sorted_by(|a, b| rules_lookup.contains_key(a) && rules_lookup[a].contains(b))
            {
                part_a += x[x.len() / 2];
            } else {
                let not_good_report = x
                    .iter()
                    .sorted_by(|a, b| {
                        (rules_lookup.contains_key(a) && rules_lookup[a].contains(b)).cmp(&true)
                    })
                    .collect_vec();
                part_b += not_good_report[not_good_report.len() / 2];
            }
        });
    (part_a, part_b)
}

#[cfg(test)]
mod day5_tests {
    use super::*;
    #[test]
    pub fn test_with_sample_data() {
        let solution = solve_part_a_and_b("./inputs/day5_sample.txt");
        assert_eq!(solution.0, 143);
        assert_eq!(solution.1, 123);
    }

    #[test]
    pub fn test_with_actual_data() {
        let solution = solve_part_a_and_b("./inputs/day5.txt");
        assert_eq!(solution.0, 5091);
        assert_eq!(solution.1, 4681);
    }
}
