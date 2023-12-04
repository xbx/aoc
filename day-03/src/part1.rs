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
            line_string.push('.');
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

        total += parse_line(previous_line, current_line, next_line);
        previous_line = &current_line;
    }

    Ok(String::from(total.to_string()))
}


fn parse_line(previous_line: &str, current_line: &str, next_line: &str) -> isize {
    //println!("prev: [{}] curr: [{}] next: [{}]", previous_line, current_line, next_line);

    let mut total = 0;
    let chars: Vec<_> = current_line.chars().collect();
    let mut current_number = String::from("");
    let mut valid_number = false;

    let mut current_pos = 0;
    for char in chars {

        if is_digit(&char) {
            current_number.push(char);
            if is_valid_region(
                current_pos, previous_line, current_line, next_line
            ) {
                valid_number = true;
            }
        } else {
            let number = parse_number(&current_number);

            if number > 0 && valid_number {
                total += number;
            }
            current_number.clear();
            valid_number = false;
        }
        current_pos += 1;
    }

    total
}

fn is_valid_region(
    current_pos: usize,
    previous_line: &str,
    current_line: &str,
    next_line: &str
) -> bool {
    if any_symbol_near(current_pos, current_line) ||
        any_symbol_near(current_pos, previous_line) ||
        any_symbol_near(current_pos, next_line) {
        true
    } else {
        false
    }
}

fn any_symbol_near(pos: usize, line: &str) -> bool {
    let string_line = String::from(line);
    let chars = string_line.chars();

    let chars_vec: Vec<_> = chars.collect();

    // previous char
    if pos > 0 && pos < line.len() {
        let char = chars_vec[pos - 1];
        if is_symbol(&char) {
            return true
        }
    }
    
    // current char
    let char = chars_vec[pos];
    if is_symbol(&char) {
        return true
    }

    // next char
    if pos + 1 < line.len(){
        let char = chars_vec[pos + 1];
        if is_symbol(&char) {
            return true
        }
    }

    false
}

fn parse_number(number: &str) -> isize {
    if number.len() > 0 {
        number.parse::<isize>().unwrap()
    } else {
        -1
    }
}

fn is_digit(char: &char) -> bool {
    match char.to_digit(10) {
        None => return false,
        Some(_) => return true
    }
}

fn is_symbol(char: &char) -> bool {
    if is_digit(char) || is_dot(&char) {
        false
    } else {
        true
    }
}

fn is_dot(char: &char) -> bool {
    *char == '.'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let output = process(input);
        assert_eq!(4361.to_string(), output?);

        Ok(())
    }

    #[test]
    fn test_process2() -> miette::Result<()> {
        let input = "
.*...
..562
";
        let output = process(input.trim());
        assert_eq!(562.to_string(), output?);

        Ok(())
    }
}



// let input = "
// ....*...............*.............*..................965...*.........754..3..657.......................92..........@..838...................
// .....116.....469...498........537..666....622&............237.......*..........*....204...........242.........&...............599=..........
// ..............-................../.............283=...........283#.919.........638.*.........452.....*794......204.326...................168
// ...........................541........................544.............../183........67..903+........................*....75.512..605........
// 860..............*.........*.............455....730....*......................143................@....-.....=...366.......@......*...+59....
// ...*..............447....916.....127........+.....*.....60................./....*...............572.658...891...+...300.....674.733.........
// .768................................*.-..........522.28.......870.....-....89./......764*...........................*........%..............
// ............606....*...................798..............298...*......342......430........668.........485.270.265/..893..............=....524
// ......#.880*.......88..907.........239..........509....=......406.+..........................264........*................829........991.%...
// ....850.................%.....&913....*...........*...............302..&228.......%601....63*......414....*156.............&...310..........
// .............@....184.......=.......260..........748........................149................................895.....697.......*.66.......
// ..........483.......*....500..970.......88..941...........25.....623...436...*.....................278..........=......*......522..*........
// ................................*.........*.-.......885....*.......#....$..848........260.........-.....579.........950...........948.......
// ";