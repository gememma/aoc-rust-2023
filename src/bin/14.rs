use std::collections::VecDeque;

advent_of_code::solution!(14);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Item {
    Round,
    Cube,
    Empty,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut platform: Vec<Vec<Item>> = vec![];
    let lines = input.lines();

    for line in lines {
        let mut row = vec![];
        for char in line.chars() {
            let item = match char {
                'O' => Item::Round,
                '#' => Item::Cube,
                _ => Item::Empty,
            };
            row.push(item)
        }
        platform.push(row);
    }

    let mut total = 0;
    for x in 0..platform[0].len() {
        let mut column = vec![];
        let mut new_column = VecDeque::new();
        for y in 0..platform.len() {
            column.push(platform[y][x])
        }
        let frags = column.split(|&i| i == Item::Cube);
        for fragment in frags {
            let mut new_frag = VecDeque::new();
            // count round items
            for item in fragment {
                match item {
                    Item::Round => new_frag.push_front(Item::Round),
                    Item::Cube => unreachable!(),
                    Item::Empty => new_frag.push_back(Item::Empty),
                }
            }
            new_column.append(&mut new_frag);
            new_column.push_back(Item::Cube);
        }

        for (i, item) in new_column.iter().enumerate() {
            match item {
                Item::Round => total += platform.len() - i,
                _ => continue,
            }
        }
    }

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut platform: Vec<Vec<Item>> = vec![];
    let lines = input.lines();
    let cycles = 1000000000;

    for line in lines {
        let mut row = vec![];
        for char in line.chars() {
            let item = match char {
                'O' => Item::Round,
                '#' => Item::Cube,
                _ => Item::Empty,
            };
            row.push(item)
        }
        platform.push(row);
    }

    let mut total = 0;
    for x in 0..platform[0].len() {
        let mut column = vec![];
        let mut new_column = VecDeque::new();
        for y in 0..platform.len() {
            column.push(platform[y][x])
        }
        let frags = column.split(|&i| i == Item::Cube);
        for fragment in frags {
            let mut new_frag = VecDeque::new();
            // count round items
            for item in fragment {
                match item {
                    Item::Round => new_frag.push_front(Item::Round),
                    Item::Cube => unreachable!(),
                    Item::Empty => new_frag.push_back(Item::Empty),
                }
            }
            new_column.append(&mut new_frag);
            new_column.push_back(Item::Cube);
        }

        for (i, item) in new_column.iter().enumerate() {
            match item {
                Item::Round => total += platform.len() - i,
                _ => continue,
            }
        }
    }

    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[ignore]
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
