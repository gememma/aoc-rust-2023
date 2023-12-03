use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    // add buffer around input
    let mut lines = input.lines().map(|s| s.to_owned()).collect::<VecDeque<_>>();
    for mut line in &mut lines {
        *line = format!(".{line}.");
    }
    lines.push_front(".".repeat(lines[0].len()));
    lines.push_back(".".repeat(lines[0].len()));

    // search for numbers
    let columns = lines[0].len();
    let rows = lines.len();
    let mut candidates = vec![];
    for i in 0..rows {
        if lines[i].contains(char::is_numeric) {
            // one or more numbers in line
            let line = lines[i].chars().collect::<Vec<_>>();
            let mut current = None;
            for j in 0..columns {
                if line[j].is_numeric() {
                    match current {
                        Some((start, _)) => {
                            current = Some((start, j + 1));
                        }
                        None => {
                            current = Some((j - 1, j + 1));
                        }
                    }
                } else if let Some((start, end)) = current.take() {
                    let line1 = lines.get(i - 1).unwrap().get(start..=end).unwrap();
                    let line2 = lines.get(i).unwrap().get(start..=end).unwrap();
                    let line3 = lines.get(i + 1).unwrap().get(start..=end).unwrap();
                    candidates.push([line1, line2, line3]);
                }
            }
        }
    }

    // compute total
    let mut total = 0;
    for candidate in candidates {
        let mut is_candidate = false;
        let length = candidate[0].len() - 2;
        for i in 0..3 {
            if let Some(_) = candidate[i].find(|c: char| c != '.' && !c.is_numeric()) {
                // candidate is a part number
                is_candidate = true;
            }
        }
        if is_candidate {
            total += candidate[1]
                .get(1..length + 1)
                .unwrap()
                .parse::<u32>()
                .unwrap();
        }
    }
    Some(total)
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Coord {
    row: usize,
    column: usize,
}

pub fn part_two(input: &str) -> Option<u32> {
    // add buffer around input
    let mut lines = input.lines().map(|s| s.to_owned()).collect::<VecDeque<_>>();
    for mut line in &mut lines {
        *line = format!(".{line}.");
    }
    lines.push_front(".".repeat(lines[0].len()));
    lines.push_back(".".repeat(lines[0].len()));

    // construct vec of gears
    let mut gears: Vec<Coord> = vec![];
    for i in 0..lines.len() {
        let mut search_str = lines[i].clone();

        loop {
            match search_str.find("*") {
                Some(index) => {
                    gears.push(Coord {
                        row: i,
                        column: index,
                    });
                    search_str = search_str.replacen("*", ".", 1);
                }
                None => break,
            }
        }
    }

    // construct HashMap for numbers
    let mut numbers: HashMap<Coord, u32> = HashMap::new();
    for i in 0..lines.len() {
        let mut current = None;
        for (j, char) in lines[i].chars().enumerate() {
            if let Some(digit) = char.to_digit(10) {
                match current {
                    None => {
                        current = Some((j, j, digit));
                    }
                    Some((start, _, value)) => {
                        current = Some((start, j, value * 10 + digit));
                    }
                }
            } else if let Some((start, end, value)) = current.take() {
                for column in start..=end {
                    numbers.insert(Coord { row: i, column }, value);
                }
            }
        }
    }

    // calculate values
    let mut total = 0;
    for gear in gears {
        // check that gear has exactly 2 adjacent numbers
        let mut values = vec![];
        for i in 0..3 {
            if numbers.contains_key(&Coord {
                row: gear.row - 1 + i,
                column: gear.column - 1,
            }) {
                values.push(
                    numbers
                        .get(&Coord {
                            row: gear.row - 1 + i,
                            column: gear.column - 1,
                        })
                        .unwrap(),
                );

                if !numbers.contains_key(&Coord {
                    row: gear.row - 1 + i,
                    column: gear.column,
                }) && numbers.contains_key(&Coord {
                    row: gear.row - 1 + i,
                    column: gear.column + 1,
                }) {
                    values.push(
                        numbers
                            .get(&Coord {
                                row: gear.row - 1 + i,
                                column: gear.column + 1,
                            })
                            .unwrap(),
                    );
                }
            } else if numbers.contains_key(&Coord {
                row: gear.row - 1 + i,
                column: gear.column,
            }) {
                values.push(
                    numbers
                        .get(&Coord {
                            row: gear.row - 1 + i,
                            column: gear.column,
                        })
                        .unwrap(),
                );
            } else if numbers.contains_key(&Coord {
                row: gear.row - 1 + i,
                column: gear.column + 1,
            }) {
                values.push(
                    numbers
                        .get(&Coord {
                            row: gear.row - 1 + i,
                            column: gear.column + 1,
                        })
                        .unwrap(),
                );
            }
        }
        if values.len() == 2 {
            total += values[0] * values[1];
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[ignore]
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
