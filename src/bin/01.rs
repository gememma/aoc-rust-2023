advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut total: i32 = 0;
    let mut temp: i32;
    for line in lines {
        let v: Vec<&str> = line.matches(char::is_numeric).collect();
        temp = (v[0].to_string()+v[v.len() -1]).parse::<i32>().unwrap();
        total = total + temp;
    }
    Some(total as u32)
}

fn numberize(input: &str) -> String {
    let mut string = input.to_owned();
    let mut i = 0;
    while i <= string.len(){
        let (slice, trailing) = string.split_at(i);

        // the following horror was to deal with cases like "eightwone" => "821"
        // yes it is disgusting and an abomination etc etc etc and YES i am proud
        let new_line = slice.replace("one", "1e").replace("two", "2o").replace("three", "3e").replace("four", "4r").replace("five", "5e").replace("six", "6x").replace("seven", "7n").replace("eight", "8t").replace("nine", "9n");

        i = new_line.len();
        i += 1;

        string = new_line+trailing;
    }
    string
}
pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut total: i32 = 0;
    let mut temp: i32;
    for line in lines {
        let new_line = numberize(line);

        let v: Vec<&str> = new_line.matches(char::is_numeric).collect();
        temp = (v[0].to_string()+v[v.len() -1]).parse::<i32>().unwrap();
        total = total + temp;
    }
    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
