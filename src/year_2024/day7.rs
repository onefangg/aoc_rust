use itertools::Itertools;
use std::fs;
use std::path::Path;

// all credit to reddit/user/Verulean314 for this smart solution
// https://publish.reddit.com/embed?url=https://www.reddit.com/r/adventofcode/comments/1h8l3z5/comment/m0tv6di/
// and reddit/user/SuperSmurfen for the smart way to derive number of digits
// https://publish.reddit.com/embed?url=https://www.reddit.com/r/adventofcode/comments/1h8l3z5/comment/m0u7ww4/
pub fn recursive_check(test_value: i64, values: &Vec<i64>, part_two: bool) -> bool {
    let last_idx = values.len() - 1;
    let check_value = values[last_idx];
    if values.len() == 1 {
        return test_value == check_value;
    }
    let other_values = values[0..(last_idx)].to_vec();
    let (quotient, remainder) = (test_value / check_value, test_value % check_value);
    if remainder == 0 && recursive_check(quotient, &other_values, part_two) {
        return true;
    }
    if part_two
        && test_value.to_string().ends_with(&check_value.to_string())
        && recursive_check(
            test_value / (10_i64.pow(check_value.ilog10() + 1)),
            &other_values,
            part_two,
        )
    {
        return true;
    }

    recursive_check(test_value - check_value, &other_values, part_two)
}

pub fn solve_part_a(input_str: &str, should_check_for_trun: bool) -> i64 {
    let content = fs::read_to_string(Path::new(input_str)).unwrap();
    let equations = content
        .lines()
        .map(|x| {
            let parsing_values = x.split_once(":").unwrap();
            (
                parsing_values.0.parse::<i64>().unwrap(),
                parsing_values
                    .1
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect_vec(),
            )
        })
        .filter(|(test_val, rhs)| {
            let check_if = recursive_check(test_val.clone(), &rhs, should_check_for_trun);
            check_if
        })
        .map(|(test_val, _)| test_val)
        .sum::<i64>();
    equations
}

#[cfg(test)]
mod day7_tests {
    use super::*;

    #[test]
    pub fn test_with_sample_data() {
        assert_eq!(solve_part_a("./inputs/day7_sample.txt", false), 3749);
        assert_eq!(solve_part_a("./inputs/day7_sample.txt", true), 11387);
    }

    #[test]
    pub fn test_with_actual_data() {
        assert_eq!(solve_part_a("./inputs/day7.txt", false), 5540634308362);
        assert_eq!(solve_part_a("./inputs/day7.txt", true), 472290821152397);
    }
}
