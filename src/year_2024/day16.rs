use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;

// this is wholesale referenced from: https://github.com/AxlLind/AdventOfCode2024/blob/main/src/bin/16.rs
// had a bug i couldn't figure out for my part 2 in c# implementation

pub fn solve(input_str: &str) -> (i64, i64) {
    let content = fs::read_to_string(Path::new(input_str)).unwrap();
    let grid = content
        .lines()
        .map(|x| x.chars().collect_vec())
        .collect_vec();

    let (mut start, mut end) = ((0, 0), (0, 0));
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == 'S' {
                start = (r, c);
            } else if grid[r][c] == 'E' {
                end = (r, c);
            }
        }
    }

    let mut p1 = i64::MAX;
    let mut visited = HashMap::new();
    let mut q = BinaryHeap::from([(0, 0, start)]);
    while let Some((score, d, (r, c))) = q.pop() {
        let norm_score = -score;
        if (r, c) == end {
            if norm_score > p1 {
                break;
            }
            p1 = norm_score;
        }

        for dir in 0..4 {
            let (dr, dc) = [(1, 0), (0, 1), (-1, 0), (0, -1)][dir];
            if r as isize + dr < 0 || c as isize + dc < 0 {
                continue;
            }
            let (rr, cc) = ((r as isize + dr) as usize, (c as isize + dc) as usize);
            if grid[rr][cc] == '#' {
                continue;
            }
            let new_score = norm_score + if d == dir { 1 } else { 1001 };
            let prev_score = *visited.get(&(rr, cc, dir)).unwrap_or(&i64::MAX);
            if new_score <= prev_score {
                visited.insert((rr, cc, dir), new_score);
                q.push((-new_score, dir, (rr, cc)));
            }
        }
    }

    let mut p2 = HashSet::new();
    let mut invert = VecDeque::new();
    for d in 0..4 {
        if *visited.get(&(end.0, end.1, d)).unwrap_or(&i64::MAX) == p1 {
            invert.push_back((end, d, p1));
        }
    }
    while let Some(((r, c), d, s)) = invert.pop_front() {
        p2.insert((r, c));
        for dir in 0..4 {
            let backtrack_score = s - if d == dir { 1 } else { 1001 };
            let (dr, dc) = [(1, 0), (0, 1), (-1, 0), (0, -1)][d];
            if r as isize - dr < 0 || c as isize - dc < 0 {
                continue;
            }
            let (rr, cc) = ((r as isize - dr) as usize, (c as isize - dc) as usize);

            if visited.get(&(rr, cc, dir)).copied().unwrap_or(i64::MAX) == backtrack_score {
                invert.push_back(((rr, cc), dir, backtrack_score));
            }
        }
    }

    (p1, p2.len() as i64 + 1)
}
#[cfg(test)]
mod day16_tests {
    use super::*;
    #[test]
    pub fn test_with_sample_data() {
        let output = solve("./inputs/day16_sample.txt");
        assert_eq!(output, (7036, 45));
    }
    #[test]
    pub fn test_with_actual_data() {
        let output = solve("./inputs/day16.txt");
        assert_eq!(output, (160624, 692));
    }
}
