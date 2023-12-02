advent_of_code::solution!(2);

#[derive(Debug)]
struct Game {
    id: u32,
    blue: u32,
    green: u32,
    red: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut games: Vec<Game> = Vec::new();

    for line in lines {
        // line = eg. "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        let v0: Vec<&str> = line.split([':']).collect();
        let (_, id) = v0[0].split_at(5);
        let mut game = Game {
            id: id.parse::<u32>().unwrap(),
            blue: 0,
            green: 0,
            red: 0,
        };
        let v: Vec<&str> = v0[1].split([';']).collect();

        for turn in v {
            // turn = eg. " 3 blue, 4 red"
            let colours: Vec<&str> = turn.split([',']).collect();
            for colour in colours {
                // colour = eg. " 3 blue"
                let c: Vec<&str> = colour.trim().split([' ']).collect();
                match c[1].trim() {
                    "blue" => {
                        let new = c[0].parse::<u32>().unwrap();
                        if game.blue < new {
                            game.blue = new;
                        }
                    }
                    "green" => {
                        let new = c[0].parse::<u32>().unwrap();
                        if game.green < new {
                            game.green = new;
                        }
                    }
                    "red" => {
                        let new = c[0].parse::<u32>().unwrap();
                        if game.red < new {
                            game.red = new;
                        }
                    }
                    other => {
                        panic!("{other}")
                    }
                }
            }
        }
        games.push(game);
    }

    let mut total = 0;
    for i in games {
        if !(i.blue > 14 || i.green > 13 || i.red > 12) {
            total += i.id;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut games: Vec<Game> = Vec::new();

    for line in lines {
        // line = eg. "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        let v0: Vec<&str> = line.split([':']).collect();
        let (_, id) = v0[0].split_at(5);
        let mut game = Game {
            id: id.parse::<u32>().unwrap(),
            blue: 0,
            green: 0,
            red: 0,
        };
        let v: Vec<&str> = v0[1].split([';']).collect();

        for turn in v {
            // turn = eg. " 3 blue, 4 red"
            let colours: Vec<&str> = turn.split([',']).collect();
            for colour in colours {
                // colour = eg. " 3 blue"
                let c: Vec<&str> = colour.trim().split([' ']).collect();
                match c[1].trim() {
                    "blue" => {
                        let new = c[0].parse::<u32>().unwrap();
                        if game.blue < new {
                            game.blue = new;
                        }
                    }
                    "green" => {
                        let new = c[0].parse::<u32>().unwrap();
                        if game.green < new {
                            game.green = new;
                        }
                    }
                    "red" => {
                        let new = c[0].parse::<u32>().unwrap();
                        if game.red < new {
                            game.red = new;
                        }
                    }
                    other => {
                        panic!("{other}")
                    }
                }
            }
        }
        games.push(game);
    }

    let mut total = 0;
    for i in games {
        let power = i.blue * i.green * i.red;
        total += power
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
