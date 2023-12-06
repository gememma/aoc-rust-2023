use std::str::FromStr;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .flat_map(u32::from_str);
    let dists = lines
        .next()
        .unwrap()
        .split_whitespace()
        .flat_map(u32::from_str);

    let races = times.zip(dists);

    let mut total = 1;
    for (time, dist) in races {
        let mut ways = 0;
        for ms in 0..=time {
            let travel = (time - ms) * ms;
            if travel > dist {
                ways += 1;
            }
        }
        total *= ways;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_once(" ")
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let dist = lines
        .next()
        .unwrap()
        .split_once(" ")
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    let mut ways = 0;
    for ms in 0..=time {
        let travel = (time - ms) * ms;
        if travel > dist {
            ways += 1;
        }
    }

    Some(ways as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[ignore]
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
