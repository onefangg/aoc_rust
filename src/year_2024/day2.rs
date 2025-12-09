use std::fs;
use std::path::Path;

pub fn solve_part_a(input_path: &str) -> usize {
    let content = fs::read_to_string(&Path::new(input_path)).unwrap();
    let safe_reports = content
        .lines()
        .filter(|x| {
            let levels_diff = x
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
                .windows(2)
                .map(|x| x[1] - x[0])
                .collect::<Vec<i32>>();
            levels_diff.iter().all(|x| x.abs() <= 3 && x.abs() > 0)
                && (levels_diff.iter().all(|x| x > &0) || levels_diff.iter().all(|x| x < &0))
        })
        .collect::<Vec<_>>();
    safe_reports.iter().count()
}

pub fn solve_part_b(input_path: &str) -> usize {
    let content = fs::read_to_string(&Path::new(input_path)).unwrap();
    let safe_reports = content
        .lines()
        .filter(|x| {
            let levels = x
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let current_levels_diff = levels
                .clone()
                .windows(2)
                .map(|x| x[1] - x[0])
                .collect::<Vec<i32>>();
            let is_report_safe = current_levels_diff
                .iter()
                .all(|x| x.abs() <= 3 && x.abs() > 0)
                && (current_levels_diff.iter().all(|x| x > &0)
                    || current_levels_diff.iter().all(|x| x < &0));

            match is_report_safe {
                true => true,
                false => (0..levels.len()).any(|idx| {
                    let mut excl_levels_diff = levels.clone();
                    excl_levels_diff.remove(idx);
                    excl_levels_diff = excl_levels_diff
                        .windows(2)
                        .map(|x| x[1] - x[0])
                        .collect::<Vec<i32>>();
                    let double_check = excl_levels_diff.iter().all(|x| x.abs() <= 3 && x.abs() > 0)
                        && (excl_levels_diff.iter().all(|x| x > &0)
                            || excl_levels_diff.iter().all(|x| x < &0));
                    double_check
                }),
            }
        })
        .collect::<Vec<_>>();
    safe_reports.iter().count()
}

#[cfg(test)]
mod day2_tests {
    use super::*;
    #[test]
    pub fn test_with_sample_data() {
        assert_eq!(solve_part_a("./inputs/day2_sample.txt"), 2);
        assert_eq!(solve_part_b("./inputs/day2_sample.txt"), 4);
    }

    #[test]
    pub fn test_with_actual_data() {
        assert_eq!(solve_part_a("./inputs/day2.txt"), 442);
        assert_eq!(solve_part_b("./inputs/day2.txt"), 493);
    }
}
