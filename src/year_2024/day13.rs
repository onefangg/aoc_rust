use itertools::Itertools;
use std::fs;
use std::path::Path;

struct EquationSolver {
    matrix: [i64; 4],
    prize: [i64; 2],
}

impl EquationSolver {
    fn solve_for_n(&self) -> i64 {
        let inverse_factor: f64 =
            1f64 / (self.matrix[0] * self.matrix[3] - self.matrix[1] * self.matrix[2]) as f64;

        let inverse_matrix = [
            self.matrix[3] as f64,
            -1f64 * self.matrix[1] as f64,
            -1f64 * self.matrix[2] as f64,
            self.matrix[0] as f64,
        ];

        let solve_for_a = inverse_factor * (inverse_matrix[0] * self.prize[0] as f64)
            + (inverse_factor * inverse_matrix[1] * self.prize[1] as f64);
        let solve_for_b = inverse_factor * (inverse_matrix[2] * self.prize[0] as f64)
            + (inverse_factor * inverse_matrix[3]) * self.prize[1] as f64;

        if (solve_for_b.round() - solve_for_b).abs() < 0.001
            && (solve_for_a.round() - solve_for_a).abs() < 0.001
        {
            return (3f64 * solve_for_a + solve_for_b).round() as i64;
        }
        0i64
    }
}

pub fn solve(input_str: &str, offset: i64) -> i64 {
    let content = fs::read_to_string(Path::new(input_str)).unwrap();

    let mut cost = 0i64;
    for machine in content.split("\r\n\r\n") {
        // found a smarter parsing solution by u/SuperSmurfen
        // https://publish.reddit.com/embed?url=https://www.reddit.com/r/adventofcode/comments/1hd4wda/comment/m1tff8b/\
        let (x1, y1, x2, y2, xr, yr) = machine
            .split(|x: char| !x.is_ascii_digit())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();

        let solver = EquationSolver {
            matrix: [x1, x2, y1, y2],
            prize: [xr + offset, yr + offset],
        };
        let res = solver.solve_for_n();
        cost += res;
    }
    cost
}

#[cfg(test)]
mod day13_tests {
    use super::*;
    #[test]
    pub fn test_with_sample_data() {
        assert_eq!(solve("./inputs/day13_sample.txt", 0), 480);
    }
    #[test]
    pub fn test_with_actual_data() {
        assert_eq!(solve("./inputs/day13.txt", 0), 30973);
        assert_eq!(solve("./inputs/day13.txt", 10000000000000), 95688837203288);
    }
}
