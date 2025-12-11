use std::ops::RangeInclusive;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut splitter_found = false;
    let mut fresh_fruits = 0;
    for line in input.lines() {
        if line.is_empty() {
            splitter_found = true;
            continue;
        }
        if !splitter_found {
            let range: Vec<&str> = line.split("-").collect();
            let from: u64 = range[0].parse().unwrap();
            let to: u64 = range[1].parse().unwrap();
            ranges.push((from, to));
        } else {
            let id: u64 = line.parse().unwrap();
            if ranges.iter().any(|(from, to)| id >= *from && id <= *to) {
                fresh_fruits += 1;
            }
        }
    }
    Some(fresh_fruits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let current_range: Vec<&str> = line.split("-").collect();
        let from: u64 = current_range[0].parse().unwrap();
        let to: u64 = current_range[1].parse().unwrap();
        let mut current_range = from..=to;

        'outer: loop {
            'inner: for index in 0..ranges.len() {
                if let Some(range) = ranges.get(index) {
                    if range.start() <= current_range.start() && current_range.end() <= range.end()
                    {
                        // current range inside range
                        break 'outer;
                    } else if current_range.start() < range.start()
                        && current_range.end() >= range.start()
                        && current_range.end() <= range.end()
                    {
                        // current range left bigger than other range
                        current_range = *current_range.start()..=*range.end();
                        ranges.remove(index);
                        break 'inner;
                    } else if current_range.end() > range.end()
                        && current_range.start() >= range.start()
                        && current_range.start() <= range.end()
                    {
                        // current range right bigger than other range
                        // ranges.insert(index, *range.start()..=*current_range.end());
                        current_range = *range.start()..=*current_range.end();
                        ranges.remove(index);
                        break 'inner;
                    } else if current_range.start() <= range.start()
                        && current_range.end() >= range.end()
                    {
                        ranges.remove(index);
                        break 'inner;
                    }
                }
            }
            if !ranges.contains(&current_range) {
                ranges.push(current_range.clone());
            }
        }
    }
    let sum: u64 = ranges
        .iter()
        .map(|range| 1 + (range.end() - range.start()))
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
