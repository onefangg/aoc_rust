use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

pub fn solve_part_a_and_b(input_str: &str) -> (usize, usize) {
    let content = fs::read_to_string(Path::new(input_str)).unwrap();
    let grid = content
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let (width, height) = (grid[0].len() as isize, grid.len() as isize);
    let antennas = (0..height).cartesian_product(0..width).fold(
        HashMap::<char, Vec<(isize, isize)>>::new(),
        |mut acc, (r, c)| {
            let ele = grid[r as usize][c as usize];
            if ele != '.' {
                acc.entry(ele)
                    .and_modify(|e| {
                        (*e).push((r, c));
                    })
                    .or_insert(Vec::<(isize, isize)>::from([(r, c)]));
            };
            acc
        },
    );
    let mut antinodes = HashSet::<(isize, isize)>::new();
    let mut antinodes_p2 = HashSet::<(isize, isize)>::new();

    antennas.iter().for_each(|(_, values)| {
        values.iter().tuple_combinations().for_each(|(c1, c2)| {
            let row_offset = c1.0 - c2.0;
            let col_offset = c1.1 - c2.1;
            antinodes.insert((c1.0 + row_offset, c1.1 + col_offset));
            antinodes.insert((c2.0 + -row_offset, c2.1 + -col_offset));

            let freq = (height / row_offset).abs();
            (2..=freq).for_each(|f| {
                antinodes_p2.insert((c1.0 + f * row_offset, c1.1 + f * col_offset));
                antinodes_p2.insert((c2.0 + f * -row_offset, c2.1 + f * -col_offset));
            });
            antinodes_p2.insert((c1.0, c1.1));
            antinodes_p2.insert((c2.0, c2.1));
        })
    });

    let part_one = antinodes
        .iter()
        .filter(|(r, c)| (*r >= 0 && *r < height) && (*c >= 0 && *c < width))
        .count();
    antinodes_p2.extend(antinodes);
    let part_two = antinodes_p2
        .iter()
        .filter(|(r, c)| (*r >= 0 && *r < height) && (*c >= 0 && *c < width))
        .count();
    (part_one, part_two)
}

#[cfg(test)]
mod day8_test {
    use super::*;
    #[test]
    pub fn test_with_sample_data() {
        let sln = solve_part_a_and_b("./inputs/day8_sample.txt");
        assert_eq!(sln.0, 14);
        assert_eq!(sln.1, 34);
    }

    #[test]
    pub fn test_with_actual_data() {
        let sln = solve_part_a_and_b("./inputs/day8.txt");
        assert_eq!(sln.0, 426);
        assert_eq!(sln.1, 1359);
    }
}
