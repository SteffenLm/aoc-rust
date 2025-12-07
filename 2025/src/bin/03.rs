advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let joltages: u64 = to_banks(input)
        .iter()
        .map(|bank| -> u64 {
            let mut first = 0u8;
            let mut second = 0u8;
            for (index, battery) in bank.iter().enumerate() {
                if battery > &first && index < bank.len() - 1 {
                    first = *battery;
                    second = 0;
                    continue;
                }
                if battery > &second {
                    second = *battery;
                }
            }
            let value = format!("{}{}", first, second);
            value.parse().expect("Should be a nice number!")
        })
        .sum();
    Some(joltages)
}

pub fn part_two(input: &str) -> Option<u64> {
    let joltages = to_banks(input).into_iter().map(|bank| -> u64 {
        let mut numbers = String::new();
        let mut left_index = 0;
        while numbers.len() < 12 {
            let last_max_index = bank.len() - (12 - numbers.len());
            let max_number = bank[left_index..=last_max_index]
                .iter()
                .max()
                .expect("Should contain a valid integer number!");
            for (index, battery) in bank
                .iter()
                .enumerate()
                .skip(left_index)
                .take(last_max_index + 1)
            {
                if battery == max_number {
                    numbers.push_str(&max_number.to_string());
                    left_index = index + 1;
                    break;
                }
            }
        }
        numbers.parse().expect("Should be parsable in a u64 value.")
    });
    Some(joltages.sum())
}

fn to_banks(to_banks: &str) -> Vec<Vec<u8>> {
    to_banks
        .lines()
        .map(|bank| -> Vec<u8> {
            bank.trim()
                .split("")
                .filter(|battery| !battery.is_empty())
                .map(|battery| -> u8 { battery.parse().expect("should be an integer") })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
