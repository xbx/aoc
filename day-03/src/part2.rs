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
            let (prev_number_l, prev_number_r) = get_adjacent_number(previous_line, current_pos);
            let (curr_number_l, curr_number_r) = get_adjacent_number(current_line, current_pos);
            let (next_number_l, next_number_r) = get_adjacent_number(next_line, current_pos);

            let mut total_parts = 0;
            if prev_number_l > 1 {
                total_parts += 1;
            }
            if prev_number_r > 1 {
                total_parts += 1;
            }
            if curr_number_l > 1 {
                total_parts += 1;
            }
            if curr_number_r > 1 {
                total_parts += 1;
            }
            if next_number_l > 1 {
                total_parts += 1;
            }
            if next_number_r > 1 {
                total_parts += 1;
            }
            if total_parts == 2 {
                total += prev_number_l * prev_number_r * curr_number_l * curr_number_r * next_number_l * next_number_r;
            }
        }
        current_pos += 1;
    }

    total
}

fn get_adjacent_number(
    line: &str,
    current_pos: usize,
) -> (isize, isize) {
    let line_vec: Vec<_> = line.chars().collect();
    let mut left_number = 1;
    let mut right_number = 1;

    if !is_digit(&line_vec[current_pos - 1])
        && !is_digit(&line_vec[current_pos])
        && !is_digit(&line_vec[current_pos + 1]) {
        return (1, 1)
    }

    let mut left_most: usize;
    let mut right_most: usize;
    
    if is_digit(&line_vec[current_pos]) {
        left_most = loop_left_most(&line_vec, current_pos);
        right_most = loop_right_most(&line_vec, current_pos);
        right_number = parse_number(&line[left_most..right_most]);
    } else if is_digit(&line_vec[current_pos - 1]) && is_digit(&line_vec[current_pos + 1]) {
        // get from the left
        right_most = current_pos;
        left_most = loop_left_most(&line_vec, current_pos - 1);
        left_number = parse_number(&line[left_most..right_most]);

        // get from the right
        left_most = current_pos + 1;
        right_most = loop_right_most(&line_vec, current_pos + 1);
        right_number = parse_number(&line[left_most..right_most]);

    } else if is_digit(&line_vec[current_pos - 1]) {
        right_most = current_pos;
        left_most = loop_left_most(&line_vec, current_pos - 1);
        left_number = parse_number(&line[left_most..right_most]);
    } else if is_digit(&line_vec[current_pos + 1]) {
        left_most = current_pos + 1;
        right_most = loop_right_most(&line_vec, current_pos + 1);
        right_number = parse_number(&line[left_most..right_most]);
    }

    (left_number, right_number)
}

fn loop_left_most(line: &Vec<char>, current_pos: usize) -> usize {
    let mut left_most = current_pos;
    loop {
        if left_most == 0 { break }
        if !is_digit(&line[left_most]) {
            break;
        } else {
            left_most -= 1;
        }
    }

    left_most
}

fn loop_right_most(line: &Vec<char>, current_pos: usize) -> usize {
    let mut right_most = current_pos;
    loop {
        if right_most == line.len() {
            right_most = line.len() - 1;
            break
        }
        if !is_digit(&line[right_most]) {
            break;
        } else {
            right_most += 1;
        }
    }

    right_most
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
