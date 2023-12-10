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

    let result = walk(&map, s_location.0, s_location.1);
    Ok(result.to_string())
}

fn walk(map: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut map_clean: Vec<Vec<char>> = vec![];
    map.iter().for_each(|line| {
        map_clean.push(line.iter().map(|_| ' ').collect());
    });

    let mut curr_x = x;
    let mut curr_y = y ;
    let mut prev_x = 0;
    let mut prev_y = 0;
    loop {
        let ((result_x, result_y), direction) = get_next(map, curr_x, curr_y, (prev_x, prev_y));
        map_clean[result_y][result_x] = map[curr_y][curr_x];
        if map[result_y][result_x] == 'S' {
            break;
        }

        // d -> left
        // l -> up
        // u -> right
        // r -> down
        match direction {
            'd' => {
                if result_x > 0 {
                    map_clean[result_y][result_x - 1] = mark_interior(&map_clean,result_y, result_x - 1)
                }
            },
            'l' => {
                if result_y > 0 {
                    map_clean[result_y - 1][result_x] = mark_interior(&map_clean,result_y - 1, result_x)
                }
            },
            'u' => {
                if result_x < map_clean[result_y].len() - 1 {
                    map_clean[result_y][result_x + 1] = mark_interior(&map_clean,result_y, result_x + 1)
                }
            },
            'r' => {
                if result_y < map_clean.len() - 1 {
                    map_clean[result_y + 1][result_x] = mark_interior(&map_clean,result_y + 1, result_x) 
                }
            },
            _ => { panic!("no more direction options") }
        }

        (prev_x, prev_y) = (curr_x, curr_y);
        (curr_x, curr_y) = (result_x, result_y); 
    }

    loop {
        let mut marked = false;
        for i in 0..map_clean.len() {
            for j in 0..map_clean[i].len() {
                if map_clean[i][j] == 'I' {
                    if j < map_clean[i].len() - 1 && map_clean[i][j+1] == ' ' {
                        map_clean[i][j+1] = 'I';
                        marked = true;
                    }
                    if i < map_clean.len() - 1 && map_clean[i+1][j] == ' ' {
                        map_clean[i+1][j] = 'I';
                        marked = true;
                    }
                    if j > 0 && map_clean[i][j-1] == ' ' {
                        map_clean[i][j-1] = 'I';
                        marked = true;
                    }
                    if i > 0 && map_clean[i-1][j] == ' ' {
                        map_clean[i-1][j] = 'I';
                        marked = true;
                    }
                }
            }
        }
        if marked == false {
            break
        }
    }
    
    map_clean.iter().for_each(|line| {
        let str = String::from_iter(line.iter());
        println!("{}", str);
    });

    let mut interiors = 0;
    map_clean.iter().for_each(|line| {
        line.iter().for_each(|c| {
            if *c == 'I' {
                interiors += 1;
            }
        })
    });
    
    interiors
}

fn mark_interior(map_clean: &Vec<Vec<char>>, y: usize, x: usize) -> char {
    if map_clean[y][x] != ' '  {
        map_clean[y][x]
    } else {
        return 'I'
    }
}

fn get_next(map: &Vec<Vec<char>>, x: usize, y: usize, previous: (usize, usize)) -> ((usize, usize), char) {
    let current = map[y][x];
    
    if ['L', '-', 'F', 'S'].contains(&current) && x < map[0].len() - 1 {
        let right = map[y][x + 1];
        if ['J', '7', '-', 'S'].contains(&right) {
            let new = (x + 1, y);
            if new != previous {
                return (new, 'r');
            }
        }
    }

    if ['F', '|', '7', 'S'].contains(&current) && y < map.len() - 1 {
        let down = map[y + 1][x];
        if ['L', 'J', '|', 'S'].contains(&down) {
            let new = (x, y + 1);
            if new != previous {
                return (new, 'd');
            }
        }
    }

    if ['J', '-', '7', 'S'].contains(&current) && x > 0 {
        let left = map[y][x - 1];
        if ['L', 'F', '-', 'S'].contains(&left) {
            let new = (x - 1, y);
            if new != previous {
                return (new, 'l');
            }
        }
    }


    if ['L', '|', 'J', 'S'].contains(&current) && y > 0 {
        let up = map[y - 1][x];
        if ['7', 'F', '|', 'S'].contains(&up) {
            let new = (x, y - 1);
            if new != previous {
                return (new, 'u');
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
..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........
";
        let output = process(input.trim());
        assert_eq!(4.to_string(), output?);

        Ok(())
    }

    #[test]
    fn test_process2() -> miette::Result<()> {
        let input = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
        let output = process(input.trim());
        assert_eq!(10.to_string(), output?);

        Ok(())
    }

}


