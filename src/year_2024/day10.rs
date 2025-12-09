use crate::utils::matrix_utils::Matrix;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

pub fn solve(input_str: &str) -> (usize, usize) {
    let content = fs::read_to_string(Path::new(input_str)).unwrap();

    let width = content.find('\n').unwrap();
    let grid = content
        .lines()
        .flat_map(|l| {
            l.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec();
    let matrix = Matrix::new(grid, width);
    let trail_heads_indices = matrix.get_indicies_of_ele(0);
    let mut visited = HashMap::<usize, HashSet<usize>>::new();

    let mut res = 0_usize;
    for trail_head_idx in trail_heads_indices {
        let mut neighbours = Vec::<usize>::new();
        neighbours.push(trail_head_idx);

        while !neighbours.is_empty() {
            let at_idx = neighbours.pop().unwrap();
            let at_ele = matrix.get_ele_with_idx(at_idx);
            if at_ele == 9 {
                visited
                    .entry(trail_head_idx)
                    .and_modify(|e| {
                        (*e).insert(at_idx);
                    })
                    .or_insert(HashSet::from([at_idx]));
                res += 1;
            }

            let (norm_row, norm_col) = matrix.derive_row_col(at_idx);
            let adjacents = matrix.get_udlr_neighbours_2d_vec(norm_row, norm_col);
            let next_door_indices = adjacents
                .into_iter()
                .filter(|(ele, _)| {
                    let diff = ele.checked_sub(at_ele);
                    if let Some(diff) = diff {
                        if diff == 1 {
                            return true;
                        }
                        return false;
                    }
                    false
                })
                .map(|(_, i)| i)
                .collect_vec();
            neighbours.extend(next_door_indices);
        }
    }
    (visited.iter().map(|x| x.1.len()).sum(), res)
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    pub fn solve_with_sample_input() {
        let (sln1, sln2) = solve("./inputs/day10_sample.txt");
        assert_eq!(sln1, 36);
        assert_eq!(sln2, 81);
    }
    #[test]
    pub fn solve_with_actual_input() {
        let (sln1, sln2) = solve("./inputs/day10.txt");
        assert_eq!(sln1, 841);
        assert_eq!(sln2, 1875);
    }
}
