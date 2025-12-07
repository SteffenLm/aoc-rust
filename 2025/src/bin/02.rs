use std::collections::HashSet;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let parsed_ranges = parse_ranges(input);
    let mut count: u64 = 0;
    for (from, to) in parsed_ranges {
        for id in from..=to {
            let id_string = id.to_string();
            if id_string.len() % 2 == 0 {
                let first_half = &id_string[..id_string.len() / 2];
                let last_half = &id_string[id_string.len() / 2..];
                if first_half == last_half {
                    count += id;
                }
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed_ranges = parse_ranges(input);
    let mut invalid_ids: HashSet<String> = HashSet::new();
    for (from, to) in parsed_ranges {
        for id in from..=to {
            let id = id.to_string();
            let max_block_size = id.len() / 2;
            for block_size in 1..=max_block_size {
                if id.len() % block_size != 0 {
                    continue;
                }
                let block_value = &id[..block_size];
                let mut found_error = false;
                for block_index in 0..id.len() / block_size {
                    let current = &id[block_index * block_size..(block_index + 1) * block_size];
                    if current != block_value {
                        found_error = true;
                        break;
                    }
                }
                if !found_error {
                    invalid_ids.insert(id.clone());
                }
            }
        }
    }
    let mut count = 0u64;
    for id in invalid_ids {
        let val: u64 = id.parse().expect("Should be convertable to interger");
        count += val;
    }
    Some(count)
}

fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .split(",")
        .filter(|range_string| !range_string.is_empty())
        .map(|range_string| range_string.trim().split("-").collect())
        .map(|id_range: Vec<&str>| {
            let start: u64 = id_range
                .first()
                .expect("Vector should have something in index 0.")
                .parse()
                .expect("Beginning should be an Integer.");
            let end: u64 = id_range
                .get(1)
                .expect("Vector should have something in index 1.")
                .parse()
                .expect("End should be an Integer.");
            (start, end)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
