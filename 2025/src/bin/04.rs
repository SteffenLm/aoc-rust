advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let coordinates: Vec<Vec<&str>> = input
        .lines()
        .into_iter()
        .map(|line| {
            line.split("")
                .into_iter()
                .filter(|cell| !cell.is_empty())
                .collect()
        })
        .collect();
    let mut count = 0;
    let max_y = coordinates.len() - 1;
    let max_x = coordinates.first().unwrap().len() - 1;
    for y in 0..coordinates.len() {
        for x in 0..coordinates.first().unwrap().len() {
            let current = coordinates.get(y).unwrap().get(x).unwrap();
            if *current != "@" {
                continue;
            }
            let mut point_around: Vec<&str> = Vec::new();
            for x_mod in -1..=1 {
                for y_mod in -1..=1 {
                    if x_mod == 0 && y_mod == 0 {
                        continue;
                    }
                    if let Some(new_x) = x.checked_add_signed(x_mod) {
                        if let Some(new_y) = y.checked_add_signed(y_mod) {
                            if new_x <= max_x && new_y <= max_y {
                                point_around
                                    .push(coordinates.get(new_y).unwrap().get(new_x).unwrap());
                            }
                        }
                    }
                }
            }
            let adjacent_rolls_of_paper = point_around
                .into_iter()
                .filter(|character| *character == "@".to_string())
                .count();
            if adjacent_rolls_of_paper < 4 {
                count += 1;
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut coordinates: Vec<Vec<&str>> = input
        .lines()
        .into_iter()
        .map(|line| {
            line.split("")
                .into_iter()
                .filter(|cell| !cell.is_empty())
                .collect()
        })
        .collect();
    let mut count = 0;

    let max_y = coordinates.len() - 1;
    let max_x = coordinates.first().unwrap().len() - 1;
    loop {
        let mut inner_count = 0;
        for y in 0..coordinates.len() {
            for x in 0..coordinates.first().unwrap().len() {
                let current = coordinates.get(y).unwrap().get(x).unwrap();
                if *current != "@" {
                    continue;
                }
                let mut point_around: Vec<&str> = Vec::new();
                for x_mod in -1..=1 {
                    for y_mod in -1..=1 {
                        if x_mod == 0 && y_mod == 0 {
                            continue;
                        }
                        if let Some(new_x) = x.checked_add_signed(x_mod) {
                            if let Some(new_y) = y.checked_add_signed(y_mod) {
                                if new_x <= max_x && new_y <= max_y {
                                    point_around
                                        .push(coordinates.get(new_y).unwrap().get(new_x).unwrap());
                                }
                            }
                        }
                    }
                }
                let adjacent_rolls_of_paper = point_around
                    .into_iter()
                    .filter(|character| *character == "@".to_string())
                    .count();
                if adjacent_rolls_of_paper < 4 {
                    count += 1;
                    coordinates[y][x] = "x";
                    inner_count += 1;
                }
            }
        }
        if inner_count == 0 {
            break;
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
