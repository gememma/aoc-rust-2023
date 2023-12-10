use std::collections::VecDeque;
use std::str::FromStr;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut seqs: Vec<Vec<i32>> = vec![];
    for line in lines {
        seqs.push(line.split_whitespace().flat_map(i32::from_str).collect());
    }

    let mut total = 0;
    for seq in seqs {
        let mut diffs: Vec<Vec<i32>> = vec![seq.clone()];

        loop {
            let mut v = vec![];
            for i in 0..(diffs[diffs.len() - 1].len() - 1) {
                let a = diffs[diffs.len() - 1][i];
                let b = diffs[diffs.len() - 1][i + 1];
                v.push(b - a);
            }

            if v == vec![0; v.len()] {
                diffs.push(v);
                break;
            }

            diffs.push(v);
        }

        let length = diffs.len();
        diffs[length - 1].push(0);
        for i in (0..length - 1).rev() {
            let a = diffs[i][diffs[i].len() - 1];
            let b = diffs[i + 1][diffs[i + 1].len() - 1];
            diffs[i].push(a + b);
        }

        total += diffs[0][diffs[0].len() - 1];
    }

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut seqs: Vec<Vec<i32>> = vec![];
    for line in lines {
        seqs.push(line.split_whitespace().flat_map(i32::from_str).collect());
    }

    let mut total = 0;
    for seq in seqs {
        let mut diffs: Vec<VecDeque<i32>> = vec![seq.clone().into()];

        loop {
            let mut v: VecDeque<i32> = vec![].into();
            for i in 0..(diffs[diffs.len() - 1].len() - 1) {
                let a = diffs[diffs.len() - 1][i];
                let b = diffs[diffs.len() - 1][i + 1];
                v.push_back(b - a);
            }

            if v == vec![0; v.len()] {
                diffs.push(v);
                break;
            }

            diffs.push(v);
        }

        let length = diffs.len();
        diffs[length - 1].push_front(0);

        for i in (0..length - 1).rev() {
            let a = diffs[i][0];
            let b = diffs[i + 1][0];
            diffs[i].push_front(a - b);
        }

        total += diffs[0][0];
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
        assert_eq!(result, Some(114));
    }

    #[ignore]
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
