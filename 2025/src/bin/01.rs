advent_of_code::solution!(1);

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

fn to_rotation_enum(line: &str) -> Rotation {
    let rotation_char = line.get(..1).expect("Should contain rotation character");
    let distance: u16 = line
        .get(1..)
        .expect("Should contain distance values")
        .parse()
        .expect("Should be an positive Integer");
    match rotation_char {
        "L" => Rotation::Left(distance as i32),
        "R" => Rotation::Right(distance as i32),
        _ => {
            panic!("It should only contain L or R")
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = 50;
    let mut sum = 0;

    for rotation in input.lines().map(to_rotation_enum) {
        match rotation {
            Rotation::Right(distance) => {
                dial = (dial + distance).rem_euclid(100);
            }
            Rotation::Left(distance) => {
                dial = (dial - distance).rem_euclid(100);
            }
        }
        if dial == 0 {
            sum += 1;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = 50;
    let mut sum = 0;

    for rotation in input.lines().map(to_rotation_enum) {
        match rotation {
            Rotation::Right(distance) => {
                for _ in 0..distance {
                    dial = (dial + 1_i32).rem_euclid(100);
                    if dial == 0 {
                        sum += 1;
                    }
                }
            }
            Rotation::Left(distance) => {
                for _ in 0..distance {
                    dial = (dial - 1_i32).rem_euclid(100);
                    if dial == 0 {
                        sum += 1;
                    }
                }
            }
        }
    }
    Some(sum as u64)
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
        assert_eq!(result, Some(6));
    }
}
