use crate::custom_error::AocError;
use std::collections::HashSet;
use regex::Regex;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {

    let mut total = 0;
    
    let lines: Vec<_> = input.split("\n").collect();
    
    for i in 0..(lines.len()) {
        total += parse_line(&lines[i]);
    }

    Ok(String::from(total.to_string()))
}

fn parse_line(line: &str) -> isize {
    let numbers = get_numbers(line);
    let winning = numbers.0;
    let have = numbers.1;
    
    let mut points = 0;
    for winner in winning {
        let won = have.get(&winner);
        match won {
            None => continue,
            Some(_) => {
                if points == 0 {
                    points = 1;
                } else {
                    points = points * 2;
                }
            }
        }
    }

    points
}

fn get_numbers(line: &str) -> (HashSet<isize>, HashSet<isize>) {
    let re: Regex = Regex::new(" +").expect("regex err");
    let card_info: Vec<_> = line.split(": ").collect();
    let both_sides: Vec<_> = card_info[1].split(" | ").collect();

    let winning_iter = re.split(both_sides[0]);
    let winning_numbers = winning_iter.map(|num| parse_number(num));

    let have_iter = re.split(both_sides[1]);
    let have_numbers = have_iter.map(|num| parse_number(num));

    let winning: HashSet<isize> = HashSet::from_iter(winning_numbers);
    let have: HashSet<isize> = HashSet::from_iter(have_numbers);

    (winning, have)
}

fn parse_number(number_str: &str) -> isize {
    let number_str_trim = number_str.trim();
    if number_str_trim.len() > 0 {
        number_str_trim.parse::<isize>().unwrap()
    } else {
        -1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let output = process(input.trim());
        assert_eq!(13.to_string(), output?);

        Ok(())
    }

}