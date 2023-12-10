use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let lines: Vec<_> = input.split("\n").collect();

    let mut s_location: (usize, usize) = (0, 0); // (x, y)
    let mut map: Vec<Vec<char>> = vec![];
    let mut y = 0;
    for line in lines {
        let chars: Vec<_> = line.chars().collect();
        match chars.iter().position(|c| *c == 'S') {
            None => (),
            Some(x) => s_location = (x, y)
        }
        map.push(chars);
        y += 1;
    }

    let result = walk(&map, s_location.0, s_location.1) / 2;
    Ok(result.to_string())
}

fn walk(map: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut steps = 0;
    let mut curr_x = x;
    let mut curr_y = y ;
    let mut prev_x = 0;
    let mut prev_y = 0;
    loop {
        println!("{steps}");
        steps += 1;
        let (result_x, result_y) = get_next(map, curr_x, curr_y, (prev_x, prev_y));
        if map[result_y][result_x] == 'S' {
            break;
        }

        (prev_x, prev_y) = (curr_x, curr_y);
        (curr_x, curr_y) = (result_x, result_y); 
    }
    
    steps
}

fn get_next(map: &Vec<Vec<char>>, x: usize, y: usize, previous: (usize, usize)) -> (usize, usize) {
    let current = map[y][x];
    println!("c: {current}");
    
    if ['L', '|', 'J', 'S'].contains(&current) && y > 0 {
        let up = map[y - 1][x];
        if ['7', 'F', '|', 'S'].contains(&up) {
            let new = (x, y - 1);
            if new != previous {
                return new;
            }
        }
    }

    if ['F', '|', '7', 'S'].contains(&current) && y < map.len() - 1 {
        let down = map[y + 1][x];
        if ['L', 'J', '|', 'S'].contains(&down) {
            let new = (x, y + 1);
            if new != previous {
                return new;
            }
        }
    }


    if ['J', '-', '7', 'S'].contains(&current) && x > 0 {
        let left = map[y][x - 1];
        if ['L', 'F', '-', 'S'].contains(&left) {
            let new = (x - 1, y);
            if new != previous {
                return new;
            }
        }
    }

    if ['L', '-', 'F', 'S'].contains(&current) && x < map[0].len() - 1 {
        let right = map[y][x + 1];
        if ['J', '7', '-', 'S'].contains(&right) {
            let new = (x + 1, y);
            if new != previous {
                return new;
            }
        }
    }
    
    panic!("no options 🔥")
}

fn _parse_number(number_str: &str) -> isize {
    let number_str_trim = number_str.trim();
    if number_str_trim.len() > 0 {
        number_str_trim.parse::<isize>().unwrap()
    } else {
        0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "
.....
.S-7.
.|.|.
.L-J.
.....
";
        let output = process(input.trim());
        assert_eq!(4.to_string(), output?);

        Ok(())
    }

}


