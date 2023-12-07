use std::collections::HashSet;
use std::str::FromStr;

advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    winning: HashSet<u32>,
    chosen: Vec<u32>,
    score: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut cards: Vec<Card> = Vec::new();

    for line in lines {
        // let mut v = line.split_whitespace().flat_map(u32::from_str);
        let mut v = line.split(&[':', '|'][..]).skip(1);

        // v[0] is Card number, v[1] are winning, v[6] are chosen
        let winning = v
            .by_ref()
            .next()
            .unwrap()
            .split_whitespace()
            .flat_map(u32::from_str)
            .collect();
        let chosen = v
            .by_ref()
            .next()
            .unwrap()
            .split_whitespace()
            .flat_map(u32::from_str)
            .collect();
        cards.push(Card {
            winning,
            chosen,
            score: 0,
        })
    }

    let mut total: u32 = 0;
    for card in cards {
        // score each card (id and score fields ignored)
        let mut matches = 0;
        for number in card.chosen {
            if card.winning.contains(&number) {
                matches += 1;
            }
        }
        if matches > 0 {
            total += 2_u32.pow(matches - 1)
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut cards: Vec<Card> = Vec::new();

    for line in lines {
        // let mut v = line.split_whitespace().flat_map(u32::from_str);
        let mut v = line.split(&[':', '|'][..]).skip(1);

        // v[0] is Card number, v[1] are winning, v[6] are chosen
        let winning = v
            .by_ref()
            .next()
            .unwrap()
            .split_whitespace()
            .flat_map(u32::from_str)
            .collect();
        let chosen = v
            .by_ref()
            .next()
            .unwrap()
            .split_whitespace()
            .flat_map(u32::from_str)
            .collect();
        cards.push(Card {
            winning,
            chosen,
            score: 0,
        })
    }

    let total_cards = cards.len();
    for card in cards.iter_mut() {
        let mut matches = 0;
        for number in card.chosen.iter() {
            if card.winning.contains(&number) {
                matches += 1;
            }
        }
        card.score = matches;
    }

    let mut counts = vec![1; total_cards];
    for index in 0..total_cards {
        for i in 1..=cards[index].score {
            if index + 1 < total_cards {
                counts[index + i as usize] += counts[index];
            }
        }
    }

    Some(counts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[ignore]
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
