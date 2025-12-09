use std::collections::HashMap;

pub fn blink(stones: HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut copy_stones_to_blink = HashMap::<u64, usize>::new();
    for (stone, count) in stones {
        match stone {
            0 => {
                copy_stones_to_blink
                    .entry(1)
                    .and_modify(|frq| *frq += count)
                    .or_insert(count);
            }
            _ => {
                let n_digits = stone.ilog10() + 1;
                if n_digits % 2 == 0 {
                    copy_stones_to_blink
                        .entry(stone / (10_u64.pow(n_digits / 2)))
                        .and_modify(|frq| *frq += count)
                        .or_insert(count);
                    copy_stones_to_blink
                        .entry(stone % (10_u64.pow(n_digits / 2)))
                        .and_modify(|frq| *frq += count)
                        .or_insert(count);
                } else {
                    copy_stones_to_blink
                        .entry(stone * 2024)
                        .and_modify(|frq| *frq += count)
                        .or_insert(count);
                }
            }
        }
    }
    copy_stones_to_blink
}

pub fn solve(input_str: &str, n: usize) -> usize {
    let mut stones = input_str
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .fold(HashMap::new(), |mut acc, e| {
            acc.entry(e).and_modify(|frq| *frq += 1).or_insert(1_usize);
            acc
        });
    for _ in 0..n {
        stones = blink(stones);
    }
    stones.values().sum::<usize>()
}

#[cfg(test)]
mod day11_tests {
    use super::*;
    #[test]
    pub fn test_scenarios() {
        assert_eq!(solve("125 17", 6), 22);
        assert_eq!(solve("4610211 4 0 59 3907 201586 929 33750", 25), 197357);
        assert_eq!(
            solve("4610211 4 0 59 3907 201586 929 33750", 75),
            234568186890978
        );
    }
}
