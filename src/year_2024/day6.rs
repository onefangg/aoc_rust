use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn find_coord(grid_data: &Vec<Vec<char>>, find: char) -> (usize, usize) {
    for (r, row) in grid_data.iter().enumerate() {
        for (c, &col) in row.iter().enumerate() {
            if col == find {
                return (r, c);
            }
        }
    }
    // shouldn't happen
    (0, 0)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

impl GuardDirection {
    pub fn turn(self) -> GuardDirection {
        match self {
            GuardDirection::Up => GuardDirection::Right,
            GuardDirection::Right => GuardDirection::Down,
            GuardDirection::Down => GuardDirection::Left,
            GuardDirection::Left => GuardDirection::Up,
        }
    }
}

struct Guard {
    row_idx: usize,
    col_idx: usize,
    direction: GuardDirection,
    is_loopy: bool,
    visited: HashSet<(usize, usize)>,
    visited_with_direction: HashSet<(usize, usize, GuardDirection)>,
}

impl Guard {
    pub fn move_one_space(
        &mut self,
        grid: &Vec<Vec<char>>,
        oob_col: usize,
        oob_row: usize,
        should_check_for_loop: bool,
    ) -> bool {
        let mut is_turn = false;
        match &self.direction {
            GuardDirection::Up => {
                if self.row_idx == 0 {
                    return false;
                }
                if grid[self.row_idx - 1][self.col_idx] != '#' {
                    self.row_idx -= 1;
                } else {
                    is_turn = true;
                }
            }
            GuardDirection::Down => {
                if self.row_idx == (oob_row - 1) {
                    return false;
                }
                if grid[self.row_idx + 1][self.col_idx] != '#' {
                    self.row_idx += 1;
                } else {
                    is_turn = true;
                }
            }
            GuardDirection::Left => {
                if self.col_idx == 0 {
                    return false;
                }
                if grid[self.row_idx][self.col_idx - 1] != '#' {
                    self.col_idx -= 1;
                } else {
                    is_turn = true;
                }
            }
            GuardDirection::Right => {
                if self.col_idx == (oob_col - 1) {
                    return false;
                }
                if grid[self.row_idx][self.col_idx + 1] != '#' {
                    self.col_idx += 1;
                } else {
                    is_turn = true;
                }
            }
        }
        if is_turn {
            self.direction = self.direction.clone().turn();
        }
        self.visited.insert((self.row_idx, self.col_idx));
        if should_check_for_loop
            && self.visited_with_direction.contains(&(
                self.row_idx,
                self.col_idx,
                self.direction.clone(),
            ))
        {
            self.is_loopy = true;
            return false;
        }
        self.visited_with_direction
            .insert((self.row_idx, self.col_idx, self.direction.clone()));

        true
    }
}

pub fn solve_part_a_and_b(input_path: &str) -> (usize, usize) {
    let content = fs::read_to_string(&Path::new(input_path)).unwrap();
    let grid = content
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let (width, height) = (grid[0].len(), grid.len());
    let (starting_row, starting_col) = find_coord(&grid, '^');
    let mut guard = Guard {
        row_idx: starting_row,
        col_idx: starting_col,
        is_loopy: false,
        direction: GuardDirection::Up,
        visited: HashSet::from([(starting_row, starting_col)]),
        visited_with_direction: HashSet::from([(starting_row, starting_col, GuardDirection::Up)]),
    };
    loop {
        match guard.move_one_space(&grid, width, height, false) {
            true => continue,
            false => break,
        }
    }

    let loopy = guard
        .visited
        .iter()
        .filter(|(r, c)| !(r == &starting_row && c == &starting_col))
        .filter(|(visited_row, visited_col)| {
            let edited_grid = (0..height)
                .map(|row| {
                    (0..width)
                        .map(|col| {
                            if *visited_row == row && *visited_col == col {
                                return '#';
                            }
                            return grid[row][col];
                        })
                        .collect_vec()
                })
                .collect_vec();
            let mut simulate_new_guard = Guard {
                row_idx: starting_row,
                col_idx: starting_col,
                is_loopy: false,
                direction: GuardDirection::Up,
                visited: HashSet::from([(starting_row, starting_col)]),
                visited_with_direction: HashSet::from([(
                    starting_row,
                    starting_col,
                    GuardDirection::Up,
                )]),
            };
            loop {
                match simulate_new_guard.move_one_space(&edited_grid, width, height, true) {
                    true => continue,
                    false => break,
                }
            }
            simulate_new_guard.is_loopy
        })
        .count();
    (guard.visited.len(), loopy)
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    #[test]
    pub fn test_with_sample_data() {
        let solution = solve_part_a_and_b("./inputs/day6_sample.txt");
        assert_eq!(solution.0, 41);
        assert_eq!(solution.1, 6);
    }

    #[test]
    pub fn test_with_actual_data() {
        let solution = solve_part_a_and_b("./inputs/day6.txt");
        assert_eq!(solution.0, 5331);
        // takes 55 second bloody inefficient
        assert_eq!(solution.1, 1812);
    }
}
