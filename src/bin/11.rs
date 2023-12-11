use std::collections::HashSet;
use std::mem;

advent_of_code::solution!(11);

#[derive(Debug, Copy, Clone)]
struct Coord(usize, usize);

pub fn part_one(input: &str) -> Option<u32> {
    // parse stringy input
    let mut universe: Vec<Coord> = vec![];
    let mut empty_y = vec![];

    let lines = input.lines();
    for (y, line) in lines.enumerate() {
        let mut line_empty = true;
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    universe.push(Coord(x, y));
                    line_empty = false;
                }
                _ => continue,
            }
        }
        if line_empty {
            empty_y.push(y)
        }
    }

    // adjust galaxy locations for empty rows
    let mut count = 0;
    for y in empty_y {
        for galaxy in mem::take(&mut universe) {
            if galaxy.1 > y + count {
                universe.push(Coord(galaxy.0, galaxy.1 + 1))
            } else {
                universe.push(galaxy)
            }
        }
        count += 1;
    }

    // adjust galaxy locations for empty columns
    let mut non_empty_x = HashSet::new();
    let mut empty_x = vec![];
    for galaxy in &universe {
        non_empty_x.insert(galaxy.0);
    }
    for i in 0..*non_empty_x.iter().max().unwrap() {
        if !non_empty_x.contains(&i) {
            empty_x.push(i);
        }
    }

    let mut count = 0;
    for x in empty_x {
        for galaxy in mem::take(&mut universe) {
            if galaxy.0 > x + count {
                universe.push(Coord(galaxy.0 + 1, galaxy.1))
            } else {
                universe.push(galaxy)
            }
        }
        count += 1;
    }

    // calculate paths
    let mut total = 0;
    for (i, galaxy) in universe.iter().enumerate() {
        for pair in i + 1..universe.len() {
            let other = universe[pair].clone();
            total += galaxy.0.abs_diff(other.0) + galaxy.1.abs_diff(other.1);
        }
    }

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    // parse stringy input
    let mut universe: Vec<Coord> = vec![];
    let mut empty_y = vec![];

    let lines = input.lines();
    for (y, line) in lines.enumerate() {
        let mut line_empty = true;
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    universe.push(Coord(x, y));
                    line_empty = false;
                }
                _ => continue,
            }
        }
        if line_empty {
            empty_y.push(y)
        }
    }

    // adjust galaxy locations for empty rows
    let mut count = 0;
    for y in empty_y {
        for galaxy in mem::take(&mut universe) {
            if galaxy.1 > y + count {
                universe.push(Coord(galaxy.0, galaxy.1 + 999999))
            } else {
                universe.push(galaxy)
            }
        }
        count += 999999;
    }

    // adjust galaxy locations for empty columns
    let mut non_empty_x = HashSet::new();
    let mut empty_x = vec![];
    for galaxy in &universe {
        non_empty_x.insert(galaxy.0);
    }
    for i in 0..*non_empty_x.iter().max().unwrap() {
        if !non_empty_x.contains(&i) {
            empty_x.push(i);
        }
    }

    let mut count = 0;
    for x in empty_x {
        for galaxy in mem::take(&mut universe) {
            if galaxy.0 > x + count {
                universe.push(Coord(galaxy.0 + 999999, galaxy.1))
            } else {
                universe.push(galaxy)
            }
        }
        count += 999999;
    }

    // calculate paths
    let mut total = 0;
    for (i, galaxy) in universe.iter().enumerate() {
        for pair in i + 1..universe.len() {
            let other = universe[pair].clone();
            total += galaxy.0.abs_diff(other.0) + galaxy.1.abs_diff(other.1);
        }
    }

    Some(total as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }
}
