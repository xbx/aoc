use crate::custom_error::AocError;
use regex::Regex;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {

    let mut total = 0;
    
    
    let mut lines_string: Vec<String> = vec![];
    
    {
        let lines: Vec<_> = input.split("\n").collect();
        for line in lines {
            let mut line_string = String::from(line);
            line_string.push(' ');
            lines_string.push(line_string);
        }
    }

    let first_line = &lines_string[0];
    let re = Regex::new(".").unwrap();
    let stuff_line = re.replace_all(&first_line, ".");
    
    let mut previous_line: &str = &stuff_line;
    
    let total_lines = lines_string.len();
    for i in 0..(total_lines) {
        let current_line = &lines_string[i];

        let next_line: &str;
        if i < (total_lines - 1) {
            next_line = &lines_string[i + 1];
        } else { // last line
            next_line = &stuff_line;
        }

        total += parse_line(
            &sanitize(previous_line),
            &sanitize(current_line),
            &sanitize(next_line)
        );
        previous_line = &current_line;
    }

    Ok(String::from(total.to_string()))
}

fn sanitize(line: &str) -> String {
    let re = Regex::new("[^\\d\\*]").unwrap();
    let sanitized = re.replace_all(&line, " ");
    String::from(sanitized)
}

fn parse_line(previous_line: &str, current_line: &str, next_line: &str) -> isize {
    //println!("prev: [{}] curr: [{}] next: [{}]", previous_line, current_line, next_line);

    let mut total = 0;
    let chars: Vec<_> = current_line.chars().collect();

    let mut current_pos = 0;
    for char in chars {

        if is_symbol(&char) {
            let prev_number = get_adjacent_number(previous_line, current_pos);
            let curr_number = get_adjacent_number(current_line, current_pos);
            let next_number = get_adjacent_number(next_line, current_pos);

            if (prev_number != 1 && curr_number != 1)
                || (prev_number != 1 && next_number != 1)
                || (curr_number != 1 && next_number != 1)
            {
                total += prev_number * curr_number * next_number;
            }
        }
        current_pos += 1;
    }

    total
}

fn get_adjacent_number(
    line: &str,
    current_pos: usize,
) -> isize {
    let line_vec: Vec<_> = line.chars().collect();

    if !is_digit(&line_vec[current_pos - 1])
        && !is_digit(&line_vec[current_pos])
        && !is_digit(&line_vec[current_pos + 1]) {
        return 1
    }

    let mut left_most = current_pos;
    let mut right_most = current_pos;
    
    if is_digit(&line_vec[current_pos]) {
        left_most = current_pos;
        right_most = current_pos;
        loop {
            if left_most == 0 { break }
            if !is_digit(&line_vec[left_most]) {
                break;
            } else {
                left_most -= 1;
            }
        }
        loop {
            if right_most == line_vec.len() {
                right_most = line_vec.len() - 1;
                break
            }
            if !is_digit(&line_vec[right_most]) {
                break;
            } else {
                right_most += 1;
            }
        }
    } else if is_digit(&line_vec[current_pos - 1]) {
        right_most = current_pos;
        left_most = current_pos - 1;
        loop {
            if left_most == 0 { break }
            if !is_digit(&line_vec[left_most]) {
                break;
            } else {
                left_most -= 1;
            }
        }
    } else if is_digit(&line_vec[current_pos + 1]) {
        left_most = current_pos + 1;
        right_most = current_pos + 1;
        loop {
            if right_most == line_vec.len() {
                right_most = line_vec.len() - 1;
                break
            }
            if !is_digit(&line_vec[right_most]) {
                break;
            } else {
                right_most += 1;
            }
        }
    }

    let number_slice = &line[left_most..right_most];

    parse_number(number_slice)
}


fn parse_number(number: &str) -> isize {
    if number.len() > 0 {
        number.trim().parse::<isize>().unwrap()
    } else {
        1
    }
}

fn is_digit(char: &char) -> bool {
    match char.to_digit(10) {
        None => return false,
        Some(_) => return true
    }
}

fn is_symbol(char: &char) -> bool {
    if *char == '*' {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let output = process(input.trim());
        assert_eq!(467835.to_string(), output?);

        Ok(())
    }

    #[test]
    fn test_process2() -> miette::Result<()> {
        let input = "
161.462
...*...
.......
";
        let output = process(input.trim());
        assert_eq!(74382.to_string(), output?);

        Ok(())
    }
}
