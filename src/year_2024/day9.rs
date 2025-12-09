use itertools::Itertools;
use std::fs;
use std::path::Path;

pub fn solve_part_a_and_b(input_str: &str, is_for_part_two: bool) -> (u64, u64) {
    let content = fs::read_to_string(Path::new(input_str)).unwrap();
    let mut create_disk_map = Vec::<isize>::new();

    content.chars().enumerate().for_each(|(idx, x)| {
        let freq = x.to_digit(10).unwrap() as usize;
        let ele = match idx % 2 == 0 {
            true => idx.div_ceil(2) as isize,
            false => -1,
        };
        (0..freq).for_each(|_| create_disk_map.push(ele));
    });

    let mut curr_idx = create_disk_map.len() - 1;
    while curr_idx > 0 && !is_for_part_two {
        let ele = create_disk_map[curr_idx];
        let first_free_idx = match create_disk_map.iter().position(|&x| x == -1) {
            Some(idx) => idx,
            None => create_disk_map.len(),
        };
        if first_free_idx >= curr_idx {
            break;
        }
        create_disk_map.remove(curr_idx);
        create_disk_map.push(-1);
        create_disk_map.remove(first_free_idx);
        create_disk_map.insert(first_free_idx, ele);
        curr_idx -= 1;
    }

    let mut curr_file_id = create_disk_map[curr_idx];
    while curr_file_id > 0 && is_for_part_two {
        let (file_block_start_idx, file_block_end_idx) =
            find_start_end_idx_of_ele(&create_disk_map, curr_file_id);
        println!("{:?}, {:?}", file_block_start_idx, file_block_end_idx);
        let file_block_len = file_block_end_idx - file_block_start_idx + 1;
        let free_mem_indicies = consecutive_slices(
            &create_disk_map
                .iter()
                .enumerate()
                .filter(|(_, &x)| x == -1)
                .map(|(idx, _)| idx)
                .collect_vec(),
        );
        let avail_mem_block = free_mem_indicies
            .iter()
            .position(|x| x.len() >= file_block_len);

        if let Some(avail_mem_block) = avail_mem_block {
            let avail_mem_block_start = *&free_mem_indicies[avail_mem_block][0];
            let avail_mem_block_end = avail_mem_block_start + file_block_len - 1;
            println!(
                "FREE {:?}, {:?}",
                avail_mem_block_start, avail_mem_block_end
            );
            if avail_mem_block_end < file_block_end_idx {
                (file_block_start_idx..=file_block_end_idx).for_each(|x| {
                    create_disk_map.remove(x);
                    create_disk_map.insert(x, -1);
                });
                (avail_mem_block_start..=avail_mem_block_end).for_each(|x| {
                    create_disk_map.remove(x);
                    create_disk_map.insert(x, curr_file_id);
                });
            }
        }
        curr_file_id -= 1;
    }
    dbg!(&create_disk_map);
    (
        calculate_checksum(&create_disk_map),
        calculate_checksum(&create_disk_map),
    )
}

// adapted from trent on stackoverflow: https://stackoverflow.com/a/50392400
pub fn consecutive_slices(data: &Vec<usize>) -> Vec<Vec<usize>> {
    (&(0..data.len()).group_by(|&i| data[i] as usize - i))
        .into_iter()
        .map(|(_, group)| group.map(|i| data[i]).collect())
        .collect()
}

pub fn find_start_end_idx_of_ele(arr: &Vec<isize>, ele: isize) -> (usize, usize) {
    (
        arr.iter().position(|&x| x == ele).unwrap(),
        arr.iter().rposition(|&x| x == ele).unwrap(),
    )
}

pub fn calculate_checksum(arr: &Vec<isize>) -> u64 {
    arr.iter()
        .enumerate()
        .filter(|(_, &e)| e != -1)
        .map(|(i, &e)| (i as u64 * e as u64))
        .sum()
}

#[cfg(test)]
mod day9_tests {
    use super::*;

    #[test]
    pub fn test_with_sample_data() {
        // let (sln1, _) = solve_part_a_and_b("./inputs/day9_sample.txt", false);
        // assert_eq!(sln1, 1928);
        let (_, sln2) = solve_part_a_and_b("./inputs/day9_sample.txt", true);
        assert_eq!(sln2, 2858);
    }

    #[test]
    pub fn test_with_actual_data() {
        let (sln1, _) = solve_part_a_and_b("./inputs/day9.txt", false);
        // need to revisit why part 1 is SO SLOW
        assert_eq!(sln1, 6211348208140);
        let (_, sln2) = solve_part_a_and_b("./inputs/day9.txt", true);
        assert_eq!(sln2, 6239783302560);
    }
}
